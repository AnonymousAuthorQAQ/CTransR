use std::io::{self, Read};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.split_whitespace();

    let n: usize = lines.next().unwrap().parse().unwrap();
    let mut x: Vec<usize> = vec![0; N];
    let mut a: Vec<usize> = vec![0; N];

    for i in 1..n {
        x[i] = lines.next().unwrap().parse().unwrap();
    }
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.split_whitespace();
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}