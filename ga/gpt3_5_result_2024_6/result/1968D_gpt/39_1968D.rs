use std::cmp;

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t: i32 = t.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let m = cmp::min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut arr: Vec<usize> = Vec::with_capacity(n + 1);
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for _ in 0..n {
            arr.push(iter.next().unwrap().parse().unwrap());
        }

        let mut aa: Vec<usize> = Vec::with_capacity(n + 1);
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for _ in 0..n {
            aa.push(iter.next().unwrap().parse().unwrap());
        }

        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);

        for x in 1..=m {
            p1 = maxl(p1, (aa[a] * (k - x + 1) as i64 + p1_) as i64);
            p1_ += aa[a];
            a = arr[a];
            p2 = maxl(p2, (aa[b] * (k - x + 1) as i64 + p2_) as i64);
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