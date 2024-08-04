use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut x = [0; N];
    let mut a = [0; N];

    // Reading input for x
    let second_line = stdin.lock().lines().next().unwrap().unwrap();
    for (i, value) in second_line.split_whitespace().enumerate() {
        x[i + 1] = value.trim().parse().unwrap(); // Align with 1..n indexing
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Calculate the rest of array a
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Output the results for array a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let t: usize = handle.lines().next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}