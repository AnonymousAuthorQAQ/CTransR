use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Reading input values into x
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();
    for i in 1..n {
        x[i] = numbers[i - 1];
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
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}