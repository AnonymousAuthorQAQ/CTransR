use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut tokens = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
        let n: usize = tokens.next().unwrap();
        let k: usize = tokens.next().unwrap();
        let a: usize = tokens.next().unwrap();
        let b: usize = tokens.next().unwrap();

        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        for (i, line) in lines.clone().take(n).enumerate() {
            arr[i + 1] = line.parse().unwrap();
        }
        for (i, line) in lines.clone().take(n).enumerate() {
            aa[i + 1] = line.parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..k {
            p1 = p1.max(p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];

            p2 = p2.max(p2_ + aa[b]);
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}