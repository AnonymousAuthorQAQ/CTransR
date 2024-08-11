use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut n = String::new();
    input.read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut x: Vec<usize> = vec![0; n];
    let mut a: Vec<usize> = vec![0; n];
    
    let mut x_values = String::new();
    input.read_line(&mut x_values).unwrap();
    let x_values: Vec<usize> = x_values.split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    for i in 1..n {
        x[i] = x_values[i - 1];
    }
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] == a[i - 1] {
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
    
    let mut t = String::new();
    input.read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}