use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn process() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut n_input = String::new();
    input.read_line(&mut n_input).unwrap();
    let n: usize = n_input.trim().parse().unwrap();

    let mut x = [0; N];
    let mut a = [0; N];

    let mut x_input = String::new();
    input.read_line(&mut x_input).unwrap();
    let x_vec: Vec<i32> = x_input.trim().split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect();

    for i in 1..n {
        x[i] = x_vec[i - 1]; // Filling x[1] to x[n-1]
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 >= n && a[i] != a[i - 1]) {
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
    let t_input = stdin.lock().lines().next().unwrap().unwrap();
    let t: usize = t_input.trim().parse().unwrap();

    for _ in 0..t {
        process();
    }
}