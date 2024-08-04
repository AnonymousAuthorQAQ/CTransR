use std::io;

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse input");

    let mut x = [0; N];
    for i in 1..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Failed to parse input");
    }

    let mut a = [0; N];
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
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
    let t: usize = input.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        proc();
    }
}