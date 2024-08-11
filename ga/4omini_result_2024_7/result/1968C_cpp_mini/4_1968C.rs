use std::io::{self, BufRead};

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
    for i in 1..n {
        x[i] = input.split_whitespace().nth(i - 1).unwrap().parse().unwrap();
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

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let t: usize = stdin.lock().lines().next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}