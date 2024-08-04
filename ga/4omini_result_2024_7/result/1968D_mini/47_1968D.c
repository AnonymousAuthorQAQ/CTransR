use std::cmp::{max, min};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut a: usize = iter.next().unwrap().parse().unwrap();
        let mut b: usize = iter.next().unwrap().parse().unwrap();

        let m = min(n, k);
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            if let Ok(num) = value.parse() {
                arr[i + 1] = num; // Adjust because arr is 1-indexed
            }
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            if let Ok(num) = value.parse() {
                aa[i + 1] = num; // Adjust because aa is 1-indexed
            }
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 1..=m {
            p1 = max(p1, (aa[a] as i64) * (k - x + 1) + p1_);
            p1_ += aa[a] as i64;
            a = arr[a];

            p2 = max(p2, (aa[b] as i64) * (k - x + 1) + p2_);
            p2_ += aa[b] as i64;
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