use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap().parse::<i64>().unwrap());
    
    let n: usize = iterator.next().unwrap() as usize;
    let mut x: Vec<i64> = vec![0; n];
    let mut a: Vec<i64> = vec![0; n];

    for i in 1..n {
        x[i] = iterator.next().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] != a[i - 1] {
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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap().parse::<i64>().unwrap());

    let t: i64 = iterator.next().unwrap();
    for _ in 0..t {
        proc();
    }
}