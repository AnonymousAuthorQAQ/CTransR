use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    
    let mut n_str = String::new();
    handle.read_line(&mut n_str).expect("Failed to read line");
    let n: usize = n_str.trim().parse().expect("Not a number");
    
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut x_input_str = String::new();
    handle.read_line(&mut x_input_str).expect("Failed to read line");
    let x_vals: Vec<usize> = x_input_str
        .split_whitespace()
        .take(n - 1)
        .map(|s| s.parse().expect("Not a number"))
        .collect();

    for i in 1..n {
        x[i] = x_vals[i - 1]; // fill x from the input
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
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
    let mut handle = stdin.lock();
    
    let mut t_str = String::new();
    handle.read_line(&mut t_str).expect("Failed to read line");
    let t: usize = t_str.trim().parse().expect("Not a number");
    
    for _ in 0..t {
        proc();
    }
}