use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Expected a number");

    // Read in the values for x
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().expect("Expected a number"))
        .collect();

    for i in 1..n {
        x[i] = values[i - 1];
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

    // Print the result array a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!(); // New line after the output
}

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Expected a number");
    
    for _ in 0..t {
        proc();
    }
}