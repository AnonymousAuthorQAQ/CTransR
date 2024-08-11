use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    
    line.clear();
    input.read_line(&mut line).unwrap();
    let x: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let mut a = vec![0; n];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }
    
    for &val in a.iter() {
        print!("{} ", val);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}