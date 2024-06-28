use rand::prelude::*;

fn main() {
    let n = std::env::args().nth(1).unwrap().parse::<usize>().unwrap();
    let mut rng = rand::thread_rng();
    for _i in 0..n {
        for _ in 0..rng.gen_range(1..100) {
            rng.gen::<u64>();
        }
        let seed = rng.gen::<u64>();
        println!("{}", seed);
    }
}
