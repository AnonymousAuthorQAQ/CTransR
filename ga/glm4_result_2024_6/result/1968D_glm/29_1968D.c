use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    while t > 0 {
        let mut tokens = lines.next().unwrap().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let k: usize = tokens.next().unwrap().parse().unwrap();
        let a: usize = tokens.next().unwrap().parse().unwrap();
        let b: usize = tokens.next().unwrap().parse().unwrap();

        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse().unwrap();
        }
        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;
        for _ in 1..=k.min(n) {
            p1 = p1.max(1LL * aa[a] * (k - 1 + 1) + p1);
            p1 += aa[a];
            a = arr[a];
            p2 = p2.max(1LL * aa[b] * (k - 1 + 1) + p2);
            p2 += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });

        t -= 1;
    }
}