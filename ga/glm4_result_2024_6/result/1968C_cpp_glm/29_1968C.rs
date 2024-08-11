use std::io::{self, Read};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x = [0; N];
    for i in 1..n {
        io::stdin().read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();
    }

    let mut a = [0; N];
    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A as usize {
            a[i] = x[i] + j as i32 * a[i - 1];
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
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}