use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc_case(x: &[i32], n: usize) -> Vec<i32> {
    let mut a: Vec<i32> = vec![0; n];
    a[0] = x[0] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + (j as i32) * a[i - 1];
            if i + 1 < n {
                if x[i + 1] < a[i] && a[i] != a[i - 1] {
                    break;
                }
            } else if a[i] != a[i - 1] {
                break;
            }
        }
    }

    a
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut x: Vec<i32> = vec![0; n];

        for i in 0..(n - 1) {
            x[i + 1] = lines.next().unwrap().unwrap().parse().unwrap();
        }

        let result = proc_case(&x, n);
        for res in result {
            print!("{} ", res);
        }
        println!();
    }
}