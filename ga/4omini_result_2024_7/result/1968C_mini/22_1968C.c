use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read the number of integers
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Read the integer values into the x array
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .take(n - 1) // because we read 1 less than n
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 1..n {
        x[i] = nums[i - 1]; // Map input to array x (0-th index is unused)
    }

    a[0] = x[1] + 1; // Initialize a[0] based on the second element of x

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j as i32 * a[i - 1]; // Calculate a[i]

            // Check condition and break if applicable
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print the output
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!(); // Newline after printing all elements
}

fn main() {
    let mut input = String::new();
    // Read the number of test cases
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc(); // Process each test case
    }
}