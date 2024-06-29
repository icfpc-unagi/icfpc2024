fn main() {
    for i in 3123..3124 {
        let mut j = i + 1;
        for k in 2..i {
            let mut v = vec![];
            let mut tmp = i;
            while tmp != 0 {
                v.push(tmp % k);
                tmp /= k;
            }
            let mut ok = true;
            for l in 0..v.len() / 2 {
                if v[l] != v[v.len() - l - 1] {
                    ok = false;
                    break;
                }
            }
            if ok {
                j = k;
                break;
            }
        }

        println!("{} {}", i, j);
    }
}
