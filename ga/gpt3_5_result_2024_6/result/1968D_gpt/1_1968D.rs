use std::io::{self, BufRead};

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        input.read_line(&mut line).unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let k = parts[1];
        let a = parts[2];
        let b = parts[3];
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        line.clear();
        input.read_line(&mut line).unwrap();
        let arr: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        line.clear();
        input.read_line(&mut line).unwrap();
        let aa: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        let mut a = a;
        let mut b = b;

        for x in 1..=m {
            p1 = maxl(p1, aa[a - 1] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a - 1] as i64;
            a = arr[a - 1] as usize;

            p2 = maxl(p2, aa[b - 1] as i64 * (k - x + 1) as i64 + p2_);
            p2_ += aa[b - 1] as i64;
            b = arr[b - 1] as usize;
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