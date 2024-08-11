use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(x: &[i32], n: usize) {
    let mut a: [i32; N] = [0; N];   // Equivalent to int a[N]
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + (j as i32) * a[i - 1];
            if i + 1 < n {
                if x[i + 1] < a[i] && a[i] != a[i - 1] {
                    break;
                }
            } else {
                if 1 < a[i] && a[i] != a[i - 1] {
                    break;
                }
            }
        }
    }

    for value in &a[..n] {
        print!("{} ", value);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut x: Vec<i32> = vec![0; N];
        for i in 1..n {
            let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
            x[i] = value;
        }
        proc(&x, n);
    }
}