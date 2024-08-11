use std::io;

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut params = input.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let k: usize = params.next().unwrap().parse().unwrap();
        let mut a: usize = params.next().unwrap().parse().unwrap();
        let mut b: usize = params.next().unwrap().parse().unwrap();
        let m = min(n, k);

        let mut arr: Vec<usize> = vec![0; N + 1];
        let mut aa: Vec<i64> = vec![0; N + 1];

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            if i < n {
                arr[i + 1] = value.parse().unwrap();
            }
        }

        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            if i < n {
                aa[i + 1] = value.parse().unwrap();
            }
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, aa[b] * (k - x + 1) + p2_);
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