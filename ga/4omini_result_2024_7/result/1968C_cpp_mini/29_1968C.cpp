use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(n: usize, x: &[usize]) {
    let mut a: [usize; N] = [0; N];
    a[0] = x[0] + 1; // x[1] in C++ corresponds to x[0] in Rust (zero-indexed)

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!(); // Print a newline at the end
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    let t: usize = input.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = input.next().unwrap().unwrap().parse().unwrap();
        let mut x: Vec<usize> = Vec::new();
        
        // Read n-1 integers
        for _ in 1..n {
            let value: usize = input.next().unwrap().unwrap().parse().unwrap();
            x.push(value);
        }

        // Call the proc function with the input
        proc(n, &x);
    }
}