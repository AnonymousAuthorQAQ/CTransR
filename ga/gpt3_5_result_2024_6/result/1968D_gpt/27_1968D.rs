use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for x in 1..=n {
            arr[x] = iter.next().unwrap().parse().unwrap();
        }
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for x in 1..=n {
            aa[x] = iter.next().unwrap().parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        for x in 1..=m {
            p1 = max(p1, (aa[a] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = max(p2, (aa[b] * (k - x + 1) as i64) + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}