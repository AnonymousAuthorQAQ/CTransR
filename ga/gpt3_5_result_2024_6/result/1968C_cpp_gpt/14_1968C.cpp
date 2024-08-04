use std::io::{self, BufRead};
use std::io::Write;

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc(x: &Vec<i64>, n: usize) -> Vec<i64> {
    let mut a = vec![0; n];
    let mut a_val = x[1] + 1;

    a[0] = a_val;

    for i in 1..n {
        let mut j = 0;
        loop {
            a_val = x[i] + j as i64 * a[i - 1];
            if (i + 1 < n && x[i+1] < a_val) || a_val == a[i - 1] {
                break;
            }
            j += 1;
        }
        a[i] = a_val;
    }

    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();
        
        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let x: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let a = proc(&x, n);

        for val in &a {
            print!("{} ", val);
        }
    }
}