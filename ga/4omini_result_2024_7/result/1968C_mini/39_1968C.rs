use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(n: usize, x: &[i32]) -> Vec<i32> {
    let mut a = vec![0; n]; // Initialize a vector for a with size n
    a[0] = x[1] + 1; // Set the first element of a

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + (j as i32) * a[i - 1]; // Calculate a[i]
            
            // Check the given condition
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    
    a
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Read the number of test cases
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Read the value of n
        let mut x = vec![0; n]; // Initialize a vector for x with size n
        for i in 1..n {
            x[i] = lines.next().unwrap().unwrap().parse().unwrap(); // Read and store values in x
        }

        let a_values = proc(n, &x); // Call the proc function to calculate a
        for val in a_values {
            print!("{} ", val); // Print each value in a
        }
        println!(); // New line after each case
    }
}