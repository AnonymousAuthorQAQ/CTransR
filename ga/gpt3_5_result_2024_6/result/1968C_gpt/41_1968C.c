use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        proc(&mut lines);
    }
}

fn proc(lines: &mut io::Lines<io::StdinLock>) {
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = vec![0; n];
    let mut a = vec![0; n];
    
    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().trim().parse().unwrap();
    }
    
    a[0] = x[1] + 1;
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || i + 1 == n {
                break;
            }
        }
    }
    
    for i in 0..n {
        print!("{:?} ", a[i]);
    }
    println!();
}