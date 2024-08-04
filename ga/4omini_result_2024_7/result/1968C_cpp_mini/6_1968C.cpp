use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000; // Using usize for range purposes

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let n: usize;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    n = input.trim().parse().unwrap();

    // Reading values into x
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<usize> = input.trim().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 1..n {
        x[i] = values[i - 1]; // Off by one because C++ was using 1-based indexing
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}