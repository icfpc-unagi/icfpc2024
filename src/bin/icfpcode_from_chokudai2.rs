use icfpc2024::pp::preprocess;
use num::BigInt;

fn main() -> anyhow::Result<()> {
    let data = vec![
        (2, 3),
        (3, 1),
        (5, 2),
        (7, 1),
        (31, 2),
        (37, 3),
        (41, 2),
        // (1,2),(4,3),(6,1)
    ];
    let code = get_chokudai2_program("4", 1000, data); // 999998
    eprintln!("{}", code);
    let code = preprocess(&code)?;
    println!("{}", code);

    Ok(())
}

fn get_chokudai2_program(problem_id: &str, ticks: usize, data: Vec<(i32, i32)>) -> String {
    let mut data = data;
    // 1 <= a
    // 1 <= b <= 3
    data.sort_by_key(|&(a, b)| (b, a)); // make encoded num smallest
    let base = data.iter().map(|&(a, _)| a).max().unwrap() + 1;
    let mut encoded = BigInt::ZERO;
    for (a, b) in data {
        encoded *= 4;
        encoded += b;
        encoded *= base;
        encoded += a;
    }
    // beta reductions ~ ticks * (3 * data.len() + 8)
    format!(
        r#"
B$ LY
B$
    B$ vY Lf Lt
        ? B= vt {ticks}
            "solve lambdaman{problem_id} "
            B.
                B! vf B+ vt 1
                BT 2 BD B* 2 B%
                    B$
                        B$ vY Lf Lp
                            ? B= vp 0
                                0
                                B+
                                    B*
                                        B/ vt B% vp {base}
                                        B/ vp {base}
                                    B!
                                        vf
                                        B/ vp {base_x4}
                        {encoded}
                    4 "UURRDDLL"
    0
Lf B$ Lx B$ vx vx Lx B$ vf B$ vx vx
"#,
        base_x4 = base * 4
    )
}
