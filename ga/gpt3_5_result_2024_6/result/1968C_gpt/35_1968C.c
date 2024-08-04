use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: u32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut t_input = String::new();
    input.read_line(&mut t_input).unwrap();
    let t: u32 = t_input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut n_input = String::new();
        input.read_line(&mut n_input).unwrap();
        let n: usize = n_input.trim().parse().unwrap();
        
        let mut x_input = String::new();
        input.read_line(&mut x_input).unwrap();
        let x: Vec<i32> = x_input.split_whitespace()
                                .map(|s| s.parse().unwrap())
                                .collect();
        
        let mut x_iter = x.iter().skip(1);
        let mut a = vec![0; n];
        a[0] = x_iter.next().unwrap() + 1;
        
        for i in 1..n {
            for j in 0..=A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i] as i32 && a[i] != a[i - 1] as i32) {
                    break;
                }
            }
        }
        
        for i in 0..n {
            print!("{} ", a[i]);
        }
        println!();
    }
}

fn main() {
    proc();
}