use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().lock().read_line(&mut input).unwrap();
    let x: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut a = vec![0; N];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || a[i] != a[i - 1]) {
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
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}