use std::io::{self, Read};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];
    
    for i in 1..n {
        x[i] = iter.next().unwrap().parse().unwrap();
    }

    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            // Prevent overflow (you would need a better check in stateful execution)
            // The original code isn't clear on how far `j` goes, so we may need to refine logic.
            a[i] = x[i] + j * a[i - 1];
            // Use usize::MAX instead of A if range checking is needed
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
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}