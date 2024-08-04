use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let mut input_line = String::new();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let n: usize = input_line.trim().parse().unwrap();

    input_line.clear();
    io::stdin().lock().read_line(&mut input_line).unwrap();
    let values: Vec<i64> = input_line.split_whitespace()
                                      .map(|x| x.parse().unwrap())
                                      .collect();
    for i in 1..n {
        x[i] = values[i - 1];
    }

    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || a[i] != a[i - 1]) {
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
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}