use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: u64 = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x: Vec<usize> = vec![0; n];
    let mut a: Vec<u64> = vec![0; n];

    input.clear();
    io::stdin().lock().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    for i in 1..n {
        x[i] = parts.next().unwrap().parse().unwrap();
    }

    a[0] = x[1] as u64 + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] as u64 + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] as usize && a[i] != a[i - 1]) || i + 1 == n {
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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    let t: usize = iterator.next().unwrap().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}