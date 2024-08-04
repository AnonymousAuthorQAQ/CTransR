use std::io::{self, BufRead};

const N: usize = 500;
const A: i32 = 1000000000;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();
    let mut iter = input.flatten();
    
    let t: i32 = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        proc(&mut iter);
    }
}

fn proc<I: Iterator<Item = String>>(iter: &mut I) {
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut x: Vec<i32> = vec![0; N];
    let mut a: Vec<i32> = vec![0; N];

    for i in 1..n {
        x[i] = iter.next().unwrap().parse().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];

            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}