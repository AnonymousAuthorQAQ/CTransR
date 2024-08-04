use std::cmp::min;
use std::cmp::max;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i32 = -1;
        let mut i_: i32 = -1;
        let mut j: i32 = -1;
        let mut j_: i32 = -1;

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut arr: [usize; N + 1] = [0; N + 1];
        let mut aa: [i32; N + 1] = [0; N + 1];

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        for x in 1..=n {
            arr[x] = iter.next().unwrap().parse().unwrap();
        }

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        for x in 1..=n {
            aa[x] = iter.next().unwrap().parse().unwrap();
        }

        for x in 1..=m {
            p1 = max(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a];
            p2 = max(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
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