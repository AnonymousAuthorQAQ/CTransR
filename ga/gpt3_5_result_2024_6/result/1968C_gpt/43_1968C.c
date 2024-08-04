use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];
    
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let n: usize = iterator.next().unwrap().parse().unwrap();
    
    for i in 1..n {
        x[i] = iterator.next().unwrap().parse().unwrap();
    }
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (a[i] != a[i - 1]) {
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