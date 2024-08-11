use std::io;

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut x: Vec<i32> = vec![0; n];
    let mut a: Vec<i32> = vec![0; n];

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let mut iter = input.split_whitespace().skip(1);
    for i in 1..n {
        x[i] = iter.next().unwrap().parse().expect("Invalid input");
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || a[i] != a[i - 1]) {
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
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        proc();
    }
}