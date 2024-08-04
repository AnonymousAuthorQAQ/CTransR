use std::io::{self, Read};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut n = 0;
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read the value of n
    io::stdin().read_to_string(&mut String::new()).unwrap();
    let input = String::from_utf8_lossy(&input);
    let values: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    n = values[0]; // The first number is n
    for i in 1..n {
        x[i] = values[i];
    }

    a[0] = x[1] + 1; // Start of the array a

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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut inputs = input.split_whitespace();
    
    let t: usize = inputs.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}