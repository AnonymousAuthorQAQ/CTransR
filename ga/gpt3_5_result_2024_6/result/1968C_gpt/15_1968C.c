use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input_line = String::new();
    handle.read_line(&mut input_line).unwrap();
    
    let t: i32 = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        let n: usize = input_line.trim().parse().unwrap();

        input_line.clear();
        handle.read_line(&mut input_line).unwrap();
        let x: Vec<i32> = input_line.split_whitespace()
                                   .map(|num| num.parse().unwrap())
                                   .collect();

        let mut a: Vec<i32> = vec![0; n];
        a[0] = x[1] + 1;

        for i in 1..n {
            let mut j = 0;
            while j <= A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) {
                    break;
                }
                j += 1;
            }
        }

        for elem in a {
            print!("{} ", elem);
        }
        println!();
    }
}

fn main() {
    proc();
}