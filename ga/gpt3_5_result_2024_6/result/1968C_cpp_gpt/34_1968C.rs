use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let mut x: Vec<usize> = vec![0; n];
        let mut a: Vec<usize> = vec![0; n];

        let input: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        for i in 1..n {
            x[i] = input[i - 1];
        }

        a[0] = x[1] + 1;
        for i in 1..n {
            let mut j = 0;
            while j <= A {
                a[i] = x[i] + j * a[i - 1];
                if (i + 1 < n && x[i + 1] < a[i]) || (a[i] != a[i - 1]) {
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
}