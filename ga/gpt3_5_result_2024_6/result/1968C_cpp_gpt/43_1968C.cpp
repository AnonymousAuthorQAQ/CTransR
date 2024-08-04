use std::io::{self, BufRead};

const N: usize = 500;
const X: i32 = 500;
const A: i32 = 1000000000;

fn proc(mut input: impl Iterator<Item = i32>) {
    let n = input.next().unwrap();
    let mut x = vec![0; n];
    for i in 1..n {
        x[i] = input.next().unwrap();
    }
    
    let mut a = vec![0; n];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || (i + 1 == n) {
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
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap().trim().parse().unwrap());
    
    let t = iterator.next().unwrap();
    for _ in 0..t {
        proc(&mut iterator);
    }
}