use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut input = handle.lines().map(|line| line.unwrap());
    
    let n: usize = input.next().unwrap().trim().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];
    
    for i in 1..n {
        x[i] = input.next().unwrap().trim().parse().unwrap();
    }
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n).then(|| x[i + 1]).unwrap_or(1) < a[i] && a[i] != a[i - 1] {
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
    let handle = stdin.lock();
    let input = handle.lines().map(|line| line.unwrap());
    let mut input_iter = input.peekable();
    
    let t: usize = input_iter.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}