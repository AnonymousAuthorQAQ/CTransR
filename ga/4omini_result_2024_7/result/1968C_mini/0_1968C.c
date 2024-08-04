use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn process() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements for this test case
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read the elements into the x array
    for i in 1..n {
        let value: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
        x[i] = value;
    }

    // Initialize a[0]
    a[0] = x[1] + 1;

    // Compute values for a
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    // Print the result for the current test case
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines = reader.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Process each test case
    for _ in 0..t {
        process();
    }
}