use std::io;

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
    let mut iter = input.split_whitespace();
    let t: i32 = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut a: usize = iter.next().unwrap().parse().unwrap();
        let mut b: usize = iter.next().unwrap().parse().unwrap();

        let m = min(n as i32, k as i32) as usize;
        let mut arr: Vec<i32> = vec![0; n + 1];
        let mut aa: Vec<i64> = vec![0; n + 1];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        iter = input.split_whitespace();
        for i in 1..=n {
            arr[i] = iter.next().unwrap().parse().unwrap();
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        iter = input.split_whitespace();
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k as i64 - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a] as usize;

            p2 = maxl(p2, aa[b] * (k as i64 - x as i64 + 1) + p2_);
            p2_ += aa[b];
            b = arr[b] as usize;
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