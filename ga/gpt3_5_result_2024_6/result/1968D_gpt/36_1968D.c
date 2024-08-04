use std::cmp::{min, max};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let k: usize = values.next().unwrap().parse().unwrap();
        let a: usize = values.next().unwrap().parse().unwrap();
        let b: usize = values.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut arr: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut aa: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        for x in 0..m {
            p1 = max(p1, (aa[a] * (k - x + 1) as usize + p1_) as i64);
            p1_ += aa[a];
            a = arr[a];
            p2 = max(p2, (aa[b] * (k - x + 1) as usize + p2_) as i64);
            p2_ += aa[b];
            b = arr[b];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}