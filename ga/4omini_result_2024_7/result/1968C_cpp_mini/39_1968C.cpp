use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut x = vec![0; n];
        let mut a = vec![0; n];
        
        // Read x values
        for i in 1..n {
            x[i] = lines.next().unwrap().unwrap().parse().unwrap();
        }
        
        a[0] = x[1] + 1; // Initialize a[0]
        
        for i in 1..n {
            for j in 0..=A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n) && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
                    break;
                }
            }
        }
        
        // Print the result for current test case
        for value in &a {
            print!("{} ", value);
        }
        println!(); // New line after each test case output
    }
}