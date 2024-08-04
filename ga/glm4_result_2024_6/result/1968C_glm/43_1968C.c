use std::io::{self, BufRead};

fn proc() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let n: usize = line.trim().parse().unwrap();
    let mut x = vec![0; n];
    for i in 1..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        x[i] = line.trim().parse().unwrap();
    }
    let mut a = vec![0; n];
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        loop {
            a[i] = x[i] + j * a[i - 1];
            if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
        }
    }
    for i in a {
        print!("{} ", i);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}