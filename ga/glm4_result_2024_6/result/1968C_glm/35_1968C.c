use std::io::{self, Read};

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();
    let mut x = vec![0; n];
    for i in 1..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();
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
    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}