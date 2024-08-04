use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let arr: Vec<usize> = (0..=n).map(|_| iter.next().unwrap().parse().unwrap()).collect();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let aa: Vec<usize> = (0..=n).map(|_| iter.next().unwrap().parse().unwrap()).collect();

        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);

        for x in 1..=m {
            p1 = max(p1, aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = max(p2, aa[b] * (k - x + 1) + p2_);
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