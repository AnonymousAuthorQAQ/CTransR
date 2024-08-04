use std::io;

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1_000_000_000;

fn process() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please enter a valid number");

    let mut x = vec![0; N];
    let mut a = vec![0; N];

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<i32> = input.split_whitespace()
                                .map(|s| s.parse().expect("Parse error"))
                                .collect();
    
    // Fill the input array x
    for i in 1..n {
        x[i] = values[i - 1];
    }

    a[0] = x[1] + 1; // Initialize a[0]

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            // Check the condition to potentially break the inner loop
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Printing the resulting array a
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a valid number");

    for _ in 0..t {
        process();
    }
}