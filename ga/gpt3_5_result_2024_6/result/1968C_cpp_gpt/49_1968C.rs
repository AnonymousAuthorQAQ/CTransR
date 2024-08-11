use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: u32 = 1000000000;

fn proc() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut iterator = handle.lines();
    
    let n: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();
    let mut x = vec![0; n];
    let mut a = vec![0; n];

    let input: Vec<usize> = iterator.next().unwrap().unwrap()
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    
    for i in 1..n {
        x[i] = input[i - 1];
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || i + 1 >= n {
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
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}