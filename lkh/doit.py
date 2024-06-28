import multiprocessing.pool
import subprocess


def convert(problem_id): 
    input_path = f"../input/spaceship/spaceship{problem_id}.txt"

    coords = [
        tuple(map(int, line.strip().split()))
        for line in open(input_path).readlines() if line.strip()
    ]
    print(f"Problem {problem_id} has {len(coords)} cities")

    prob_path = f"tmp/spaceship{problem_id}.tsp"
    with open(prob_path, "w") as f:
        f.write(f"""NAME : spaceship{problem_id}
COMMENT : spaceship{problem_id} tsp
TYPE : TSP
DIMENSION : {len(coords) + 1}
EDGE_WEIGHT_TYPE : EUC_2D
NODE_COORD_SECTION
1 0 0
""")
        for i, (x, y) in enumerate(coords, start=2):
            f.write(f"{i} {x} {y}\n")
        f.write("EOF\n")

    par_path = f"tmp/spaceship{problem_id}.par"
    out_path = f"tmp/spaceship{problem_id}.out"
    with open(par_path, "w") as f:
        f.write(f"""PROBLEM_FILE = {prob_path}
OUTPUT_TOUR_FILE = {out_path}
MOVE_TYPE = 5
PATCHING_C = 3
PATCHING_A = 2
RUNS = 10""")
        
    subprocess.run(
        ["./LKH-3.0.10/LKH", par_path],
        check=True
    )
    
    with open(out_path) as f:
        lines = [
            line.strip() for line in f.readlines()
        ]
        lines = lines[lines.index("TOUR_SECTION") + 1:lines.index("-1")]
        tour = [int(line) for line in lines]
        assert tour[0] == 1
        tour = [v - 2 for v in tour[1:]]

    with open(f"tour/tour{problem_id}.txt", "w") as f:
        f.write("\n".join(map(str, tour)) + "\n")
    
    print(f"Problem {problem_id} done!!!!!")


import multiprocessing
with multiprocessing.pool.ThreadPool() as pool:
    pool.map(convert, range(12, 25))

#for i in range(12, 26):
#    print("-" * 100)
#    print(f"Problem {i}")
#    convert(i)