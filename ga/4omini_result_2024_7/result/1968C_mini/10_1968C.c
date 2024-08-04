use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read x array
    let mut x = vec![0; N];
    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    let mut a = vec![0; N];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) then { // Safe check for boundaries
                if x[i + 1] < a[i] && a[i] != a[i - 1] {
                    break;
                }
            } else {
                if 1 < a[i] && a[i] != a[i - 1] {
                    break;
                }
            }
        }
    }

    // Print result for this test case
    for i in 0..n {
        print!("{} ", a[i]);
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