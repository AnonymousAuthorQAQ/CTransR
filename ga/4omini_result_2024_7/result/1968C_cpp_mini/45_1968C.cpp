use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please type a number!");

    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read line");
    let nums: Vec<usize> = line
        .trim()
        .split_whitespace()
        .skip(1) // Skip the first skipped input since i starts from 1
        .map(|s| s.parse().expect("Please type a number!"))
        .collect();

    x[1..n].copy_from_slice(&nums[0..(n - 1)]);

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
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
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        proc();
    }
}