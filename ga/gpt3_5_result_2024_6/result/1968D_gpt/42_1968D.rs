use std::cmp::{min, max};

const N: usize = 200000;

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("failed to read input");
    let t: usize = t.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("failed to read input");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("invalid input");
        let k: usize = iter.next().unwrap().parse().expect("invalid input");
        let a: usize = iter.next().unwrap().parse().expect("invalid input");
        let b: usize = iter.next().unwrap().parse().expect("invalid input");
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut arr: Vec<usize> = Vec::with_capacity(N + 1);
        let mut aa: Vec<usize> = Vec::with_capacity(N + 1);

        input.clear();
        std::io::stdin().read_line(&mut input).expect("failed to read input");
        arr.extend(input.split_whitespace().map(|x| x.parse().unwrap()));

        input.clear();
        std::io::stdin().read_line(&mut input).expect("failed to read input");
        aa.extend(input.split_whitespace().map(|x| x.parse().unwrap()));

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a;
        let mut b;

        for x in 1..=m {
            p1 = max(p1, aa[a] * (k - x + 1) as usize + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = max(p2, aa[b] * (k - x + 1) as usize + p2_);
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