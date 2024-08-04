use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = vec![0; n];
    let mut a = vec![0; n];

    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];

            let next_value = if i + 1 < n { x[i + 1] } else { 1 };
            if next_value < a[i] && a[i] != a[i - 1] {
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
    let lines = handle.lines();
    let mut iter = lines.map(|line| line.unwrap()).peekable();
    
    let t: usize = iter.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}