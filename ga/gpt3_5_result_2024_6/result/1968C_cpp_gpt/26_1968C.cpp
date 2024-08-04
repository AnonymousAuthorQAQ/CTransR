use std::io::{self, BufRead};

const N: usize = 500;
const X: usize = 500;
const A: i64 = 1000000000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: i32 = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let x: Vec<i64> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        let mut a: Vec<i64> = vec![0; n];
        a[0] = x[1] + 1;

        for i in 1..n {
            let mut j = 0;
            while j <= A {
                a[i] = x[i] + j * a[i - 1];
                if i + 1 < n && x[i + 1] < a[i] && a[i] != a[i - 1] {
                    break;
                }
                j += 1;
            }
        }

        for val in a {
            print!("{} ", val);
        }
        println!();
    }
}