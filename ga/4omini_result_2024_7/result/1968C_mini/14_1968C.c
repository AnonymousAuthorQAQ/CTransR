use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    let n: usize = io::stdin().read_line(&mut input).unwrap().trim().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let x_values: Vec<usize> = input
        .split_whitespace()
        .take(n - 1)
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 1..n {
        x[i] = x_values[i - 1]; // Fill x array; note x[0] remains 0
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
    let t: usize;
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    t = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}