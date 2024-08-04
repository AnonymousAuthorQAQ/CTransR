use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();
    
    input.read_line(&mut buf).unwrap();
    let x: Vec<i32> = buf.split_whitespace().map(|s| s.parse().unwrap()).collect();
    
    let mut a = vec![0; N];
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
    
    let mut buf = String::new();
    input.read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}