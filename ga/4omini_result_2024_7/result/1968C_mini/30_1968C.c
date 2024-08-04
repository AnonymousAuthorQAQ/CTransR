use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc_case() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Not a number");
    
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read into array x from index 1 to n-1
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().expect("Not a number"))
        .collect();
    
    for i in 1..n {
        x[i] = numbers[i - 1];  // x[1] corresponds to first read value
    }

    a[0] = x[1] + 1; // Setting the first element of `a`

    // Main logic to compute array `a`
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];

            // Check conditions before breaking the loop
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    
    // Print the resulting array `a`
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Not a number");

    for _ in 0..t {
        proc_case();
    }
}