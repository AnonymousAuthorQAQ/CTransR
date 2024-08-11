use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read the value of n
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read the x values
    input.clear();
    reader.read_line(&mut input).unwrap();
    let x_values: Vec<usize> = input.trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    for i in 1..n {
        x[i] = x_values[i - 1];
    }
    
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print the output
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read the number of test cases
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}