use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        parts = input.trim().split_whitespace();
        for x in 1..=n {
            arr[x] = parts.next().unwrap().parse().unwrap();
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        parts = input.trim().split_whitespace();
        for x in 1..=n {
            aa[x] = parts.next().unwrap().parse().unwrap();
        }

        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);

        for x in 1..=m {
            p1 = max(p1, (aa[a] * (k - x + 1) + p1_) as i64);
            p1_ += aa[a];
            a = arr[a];
            p2 = max(p2, (aa[b] * (k - x + 1) + p2_) as i64);
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}