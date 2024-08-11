use std::cmp;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        let n: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        let a: i32 = iter.next().unwrap().parse().unwrap();
        let b: i32 = iter.next().unwrap().parse().unwrap();
        let m = cmp::min(n, k) as usize;
        let mut a_ = a;
        let mut b_ = b;

        let mut arr: Vec<i32> = vec![0; N + 1];
        let mut aa: Vec<i32> = vec![0; N + 1];

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        for x in 1..=n {
            arr[x] = iter.next().unwrap().parse().unwrap();
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        for x in 1..=n {
            aa[x] = iter.next().unwrap().parse().unwrap();
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i32 = -1;
        let mut j: i32 = -1;
        let mut i_: i32 = -1;
        let mut j_: i32 = -1;

        for x in 1..=m {
            p1 = cmp::max(p1, (aa[a as usize] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];
            p2 = cmp::max(p2, (aa[b as usize] * (k - x + 1) as i64) + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize];
        }

        let result = if p1 > p2 {
            "Bodya"
        } else if p1 < p2 {
            "Sasha"
        } else {
            "Draw"
        };

        println!("{}", result);
    }
}