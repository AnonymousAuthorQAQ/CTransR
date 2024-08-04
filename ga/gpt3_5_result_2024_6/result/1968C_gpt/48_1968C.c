use std::io::{self, BufRead};

const N: usize = 500;
const X: i32 = 500;
const A: i64 = 1000000000;

fn proc(n: usize, x: &Vec<i32>) {
    let mut a: Vec<i64> = vec![0; n];
    a[0] = x[1] as i64 + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] as i64 + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < 1) || a[i] != a[i - 1] {
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
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        
        stdin.lock().read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        input.clear();
        stdin.lock().read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        proc(n, &nums);
    }
}