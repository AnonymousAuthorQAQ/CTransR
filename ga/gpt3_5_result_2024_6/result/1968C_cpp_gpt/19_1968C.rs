use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let x: Vec<i64> = input.split_whitespace()
                            .map(|s| s.parse().unwrap())
                            .collect();

    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || (i + 1 == n) {
                break;
            }
            j += 1;
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}