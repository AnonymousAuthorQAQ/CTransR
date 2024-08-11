use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: usize = input.trim().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    
    // Read x[1] to x[n-1]
    let parts: Vec<usize> = input.trim().split_whitespace()
                                   .map(|s| s.parse().unwrap())
                                   .collect(); 

    // Populate x from index 1 to n-1
    for i in 1..n {
        x[i] = parts[i - 1]; // parts[i - 1] takes care of x[1] being the first element read
    }
    
    a[0] = x[1] + 1; // Initialize a[0]

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1]; // Update a[i] based on the formula
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] != a[i - 1] {
                break; // Break if the condition is satisfied
            }
        }
    }

    // Output the result
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}