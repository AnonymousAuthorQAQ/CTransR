use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn proc(x: &Vec<i64>) -> Vec<i64> {
    let n = x.len();
    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1]) || (i + 1 >= n) {
                break;
            }
            j += 1;
        }
    }

    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = input.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = input.next().unwrap().trim().parse().unwrap();
        let x: Vec<i64> = input.next().unwrap().trim().split_whitespace()
            .map(|num| num.parse().unwrap()).collect();

        let result = proc(&x);
        for val in result {
            print!("{} ", val);
        }
        println!();
    }
}