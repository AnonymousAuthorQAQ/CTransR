use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc(input_values: &mut std::io::StdinLock, n: usize) {
    let mut x= vec![0; N];
    let mut a = vec![0; N];
    
    for i in 1..n {
        x[i] = input_values.next().unwrap().unwrap().parse().unwrap();
    }
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j: i64 = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || a[i] != a[i - 1] {
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
    let stdin_lock = stdin.lock();
    
    let mut input_values = stdin_lock.lines().map(|x| x.unwrap());
    
    let t: usize = input_values.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = input_values.next().unwrap().parse().unwrap();
        proc(&mut input_values, n);
    }
}