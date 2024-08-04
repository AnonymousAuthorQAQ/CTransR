use std::io;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
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
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let mut a: usize = parts.next().unwrap().parse().unwrap();
        let mut b: usize = parts.next().unwrap().parse().unwrap();
        let m = min(n as i32, k as i32) as usize;

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            arr[i + 1] = val.parse().unwrap();
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            aa[i + 1] = val.parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        let mut i_ = -1;
        let mut j_ = -1;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * ((k - x + 1) as i64) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, (aa[b] as i64) * ((k - x + 1) as i64) + p2_);
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