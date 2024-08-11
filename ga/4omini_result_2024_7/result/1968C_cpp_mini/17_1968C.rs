use std::io::{self, BufRead};

const N: usize = 500;
const A: i64 = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let n: usize = input.next().unwrap().unwrap().parse().unwrap();
    let mut x: Vec<i64> = vec![0; N];
    let mut a: Vec<i64> = vec![0; N];

    for i in 1..n {
        x[i] = input.next().unwrap().unwrap().parse().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];

            if (i + 1 < n).then(|| x[i + 1])   // Condition to check for next element in the array `x`
                .unwrap_or(1) < a[i] && a[i] != a[i - 1] {
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
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}