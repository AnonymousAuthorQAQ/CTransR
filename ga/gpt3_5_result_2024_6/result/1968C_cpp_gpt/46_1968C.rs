use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();

    line.clear();
    io::stdin().lock().read_line(&mut line).unwrap();
    let mut input_iter = line.split_whitespace().skip(1);

    for i in 1..n {
        x[i] = input_iter.next().unwrap().parse().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] as usize) || a[i] != a[i - 1] as usize {
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
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}