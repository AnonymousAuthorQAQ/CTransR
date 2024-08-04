use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: u32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|line| line.unwrap());
    
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut x: Vec<u32> = Vec::with_capacity(n);
    
    for i in 1..n {
        x.push(iterator.next().unwrap().parse().unwrap());
    }
    
    let mut a: Vec<u32> = vec![0; n];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] as usize && a[i] != a[i - 1]) || (i + 1 >= n) {
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
    let mut iterator = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}