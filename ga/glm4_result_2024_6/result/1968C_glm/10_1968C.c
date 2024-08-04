use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    let mut x = [0; N];
    for i in 1..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        x[i] = line.trim().parse().unwrap();
    }
    let mut a = [0; N];
    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && a[i] != a[i - 1]) {
                break;
            }
        }
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}