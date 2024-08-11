use std::io::{self, BufRead};

const N: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read number of elements n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Create a vector for x and a
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read values into x
    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    // Set a[0]
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            // Check the condition and break if met
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    
    // Print the array a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read number of test cases t
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}