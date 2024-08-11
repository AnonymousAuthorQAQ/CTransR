use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut x: Vec<i64> = Vec::new();
    for _ in 1..n {
        x.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut a: Vec<i64> = vec![0; n];
    a[0] = x[0] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i-1] + j * a[i - 1];
            if (i + 1 < n && x[i] < a[i]) || a[i] != a[i - 1] {
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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}