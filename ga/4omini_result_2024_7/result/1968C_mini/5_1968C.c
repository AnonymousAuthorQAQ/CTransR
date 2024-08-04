use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse input");
    
    let mut x = vec![0; N];
    let mut a = vec![0; N];
    
    // Read x values
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse number"))
        .collect();
    
    for i in 1..n {
        x[i] = nums[i - 1]; // Adjust for 1-based indexing
    }
    
    a[0] = x[1] + 1; // a[0] corresponds to x[1] in C (1-based index)

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j as i32 * a[i - 1];
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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse input");
    
    for _ in 0..t {
        proc();
    }
}