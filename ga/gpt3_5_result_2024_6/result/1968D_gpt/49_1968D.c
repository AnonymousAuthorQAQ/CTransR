use std::io::{self, BufRead};

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());

    let t: i32 = input.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i32 = -1;
        let mut i_: i32 = -1;
        let mut j: i32 = -1;
        let mut j_: i32 = -1;

        let line: String = input.next().unwrap().trim().to_string();
        let mut parts = line.split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut a_: i32 = a;
        let mut b_: i32 = b;

        let mut arr: Vec<i32> = vec![0; (n + 1) as usize];
        let mut aa: Vec<i32> = vec![0; (n + 1) as usize];

        let line1: String = input.next().unwrap().trim().to_string();
        let parts1: Vec<&str> = line1.split_whitespace().collect();
        for (index, part) in parts1.iter().enumerate() {
            arr[index + 1] = part.parse().unwrap();
        }

        let line2: String = input.next().unwrap().trim().to_string();
        let parts2: Vec<&str> = line2.split_whitespace().collect();
        for (index, part) in parts2.iter().enumerate() {
            aa[index + 1] = part.parse().unwrap();
        }

        for x in 1..=m {
            p1 = maxl(p1, (aa[a as usize] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a as usize] as i64;
            a = arr[a as usize];
            p2 = maxl(p2, (aa[b as usize] * (k - x + 1) as i64) + p2_);
            p2_ += aa[b as usize] as i64;
            b = arr[b as usize];
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