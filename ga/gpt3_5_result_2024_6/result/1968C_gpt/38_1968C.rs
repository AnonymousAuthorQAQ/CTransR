use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut input = String::new();

    handle.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    input.clear();

    let mut x: Vec<usize> = vec![0; N];
    let mut a: Vec<usize> = vec![0; N];

    handle.read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.split_whitespace().collect();
    for (i, part) in parts.iter().enumerate() {
        if i > 0 {
            x[i] = part.parse().unwrap();
        }
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || a[i] != a[i - 1]) {
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
    let mut handle = stdin.lock();
    let mut input = String::new();

    handle.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    input.clear();

    for _ in 0..t {
        proc();
    }
}