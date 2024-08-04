use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().lock().read_line(&mut input).unwrap();
    let x: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut a = vec![0; N];
    let mut i = 1;
    a[0] = x[1] + 1;

    while i < n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || (i + 1 == n) {
                break;
            }
            j += 1;
        }
        i += 1;
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    
    let t: usize = iterator.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}