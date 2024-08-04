use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    // Read the number of elements
    reader.read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut x = vec![0; N]; // allocate array for x up to size N
    let mut a = vec![0; N]; // allocate array for a up to size N

    input.clear();
    
    // Read elements into x
    reader.read_line(&mut input).unwrap();
    let values: Vec<i32> = input
        .split_whitespace()
        .take(n - 1) // Read exactly n - 1 integers
        .map(|s| s.parse().unwrap())
        .collect();

    for (i, &value) in values.iter().enumerate() {
        x[i + 1] = value; // Filling x starting from index 1
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + (j as i32) * a[i - 1];

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
    let reader = stdin.lock();
    
    let mut input = String::new();
    // Read number of test cases
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}