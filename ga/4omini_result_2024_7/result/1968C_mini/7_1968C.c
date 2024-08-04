use std::io;

fn proc_case() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a number");

    let mut x: Vec<i32> = vec![0; n + 1];  // Create a vector with n+1 capacity.
    let mut a: Vec<i32> = vec![0; n];

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let x_values: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().expect("Parsing to i32 failed"))
        .collect();

    for i in 1..n {
        x[i] = x_values[i - 1];  // Filling x from index 1 to n - 1, 0 is unused.
    }

    a[0] = x[1] + 1;  // Setting a[0] to x[1] + 1

    const A: i32 = 1_000_000_000;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];

            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;  // Breaking out of the loop if conditions are met.
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);  // Print each element in a.
    }
    println!();  // Newline after each case output.
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");
    
    for _ in 0..t {
        proc_case();  // Process each test case.
    }
}