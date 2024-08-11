use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x: Vec<i32> = vec![0; n];
    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    let mut a: Vec<i32> = vec![0; n];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || a[i] != a[i - 1]) {
                break;
            }
        }
    }
    
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}  