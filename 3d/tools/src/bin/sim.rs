use tools::parse_output;

fn main() {
    let output = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let input = std::env::args()
        .skip(2)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let out = parse_output(&output).unwrap();
    let sim = tools::compute_score(&out, &input);
    for (t, s) in sim.log {
        println!("t = {}", t);
        for row in s.iter() {
            for v in row.iter() {
                print!("{:3} ", v.to_string());
            }
            println!();
        }
        println!();
    }
    if sim.err.len() > 0 {
        println!("err = {}", sim.err);
    }
    println!("output = {}", sim.ret);
    println!("score = {}", sim.score);
}
