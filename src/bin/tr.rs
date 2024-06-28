use std::io::Read;

fn tr(c: char) -> char {
    // TODO: make it a constnat
    let chars: Vec<_> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!\"#$%&'()*+,-./:;<=>?@[\\]^_`|~ \n".chars().collect();
    return chars[c as usize - 33];
}

fn main() {
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    let s = s.chars().map(tr).collect::<String>();
    println!("{}", s);
}
