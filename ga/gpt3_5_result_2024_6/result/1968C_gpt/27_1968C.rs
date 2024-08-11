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
    
    let mut x: Vec<i32> = vec![0; n];
    let mut a: Vec<i32> = vec![0; n];
    
    line.clear();
    input.read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    for i in 1..n {
        x[i] = iter.next().unwrap().parse().unwrap();
    }
    
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || i + 1 == n {
                break;
            }
            j += 1;
        }
    }
    
    for i in 0..n {
        print!("{} ", a[i]);
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