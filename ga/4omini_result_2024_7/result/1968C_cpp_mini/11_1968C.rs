use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    // Read n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    let mut x = vec![0; n]; // defining vector for x
    let mut a = vec![0; n]; // defining vector for a
    
    // Read x[1..n]
    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    
    for &value in &a {
        print!("{} ", value);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    // Read number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}