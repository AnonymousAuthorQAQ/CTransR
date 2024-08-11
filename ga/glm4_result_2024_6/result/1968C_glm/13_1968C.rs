use std::io::{self, Read};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    
    let mut x = [0; N];
    for i in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();
    }
    
    let mut a = [0; N];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && a[i] != a[i - 1]) {
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
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}