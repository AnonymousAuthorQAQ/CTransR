use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let mut x: Vec<i32> = Vec::new();
    x.push(0); // to fill the 0th index since the loop starts from 1
    for _ in 1..n {
        x.push(iterator.next().unwrap().parse().unwrap());
    }

    let mut a: Vec<i32> = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] == a[i - 1] {
                break;
            }
            j += 1;
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