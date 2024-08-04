use std::io;
use std::io::prelude::*;

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

fn maxl(a: i64, b: i64) -> i64 {
    a.max(b)
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    let t: i32 = line.trim().parse().expect("Invalid input");

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        let mut tokens = line.trim().split_whitespace();
        let n: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let k: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let a: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let b: i32 = tokens.next().unwrap().parse().expect("Invalid input");

        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];

        for i in 1..=n {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            arr[i as usize] = line.trim().parse().expect("Invalid input");
        }

        for i in 1..=n {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            aa[i as usize] = line.trim().parse().expect("Invalid input");
        }

        let m = min(n, k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for x in 1..=m {
            p1 = maxl(p1, 1i64 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = maxl(p2, 1i64 * aa[b as usize] * (k - x + 1) + p2_);
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

        writeln!(stdout, "{}", result).expect("Failed to write result");
    }
}