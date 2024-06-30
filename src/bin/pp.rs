use std::io::Read;

use icfpc2024::pp::preprocess;

fn main() -> anyhow::Result<()> {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf)?;
    let s = preprocess(&buf)?;
    println!("{}", s);

    Ok(())
}
