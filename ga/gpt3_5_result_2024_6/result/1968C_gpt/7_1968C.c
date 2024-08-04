use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc(x: &Vec<i32>) -> Vec<i32> {
    let mut n = x.len();
    let mut a: Vec<i32> = vec![0; n];
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + (j as i32) * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] as i32 && a[i] != a[i - 1]) || i + 1 == n {
                break;
            }
            j += 1;
        }
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
        let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = arr.len();
        
        let result = proc(&arr);
        for i in 0..n {
            print!("{} ", result[i]);
        }
        println!();
    }
}