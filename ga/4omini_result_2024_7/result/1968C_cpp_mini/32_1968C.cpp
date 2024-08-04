use std::io::{self, BufRead};

const N: usize = 500; // Maximum size of input
const A: usize = 1_000_000_000; // Constant value

fn proc() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut n_line = String::new();
    reader.read_line(&mut n_line).unwrap();
    let n: usize = n_line.trim().parse().unwrap();
    
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut x_line = String::new();
    reader.read_line(&mut x_line).unwrap();
    let x_values: Vec<i32> = x_line.split_whitespace()
                                    .skip(1) // Skip the first number, as per the C++ code
                                    .map(|val| val.parse().unwrap())
                                    .collect();
    
    for (i, &value) in x_values.iter().enumerate() {
        x[i + 1] = value; // Note: x[0] is unused
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j as i32 * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!(); // Print a newline
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let t_line = reader.lines().next().unwrap().unwrap();
    let t: usize = t_line.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}