use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1000000000;

fn proc(n: usize, x: Vec<i32>) -> Vec<i32> {
    let mut a: Vec<i32> = vec![0; n];
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

    a
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = input.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: usize = input.next().unwrap().trim().parse().unwrap();
        let x: Vec<i32> = input.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();

        let result = proc(n, x);
        println!("{}", result.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
    }
}