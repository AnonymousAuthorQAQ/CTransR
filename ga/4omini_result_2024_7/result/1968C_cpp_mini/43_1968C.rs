use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(test_case: &Vec<usize>) {
    let n = test_case.len();
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    for i in 1..n {
        x[i] = test_case[i - 1];
    }
    
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            let next_x = if i + 1 < n { x[i + 1] } else { 1 };
            if next_x < a[i] && a[i] != a[i - 1] {
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
    let handle = stdin.lock();
    let reader = handle.lines().map(|line| line.unwrap());
    let mut lines = reader.peekable();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let mut test_case = Vec::new();
        
        for _ in 0..(n - 1) {
            let value: usize = lines.next().unwrap().parse().unwrap();
            test_case.push(value);
        }

        proc(&test_case);
    }
}