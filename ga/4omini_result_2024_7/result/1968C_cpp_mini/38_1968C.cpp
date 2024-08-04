use std::io::{self, BufRead};

const N: usize = 500;
const A: i64 = 1_000_000_000;

fn proc(x: &[i64], n: usize) -> Vec<i64> {
    let mut a = vec![0; n];
    a[0] = x[0] + 1; // x[0] corresponds to x[1] in C++
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
                break;
            }
        }
    }
    
    a
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().parse().unwrap();
        let mut x = vec![0; n];
        
        for i in 1..n {
            x[i] = lines.next().unwrap().parse().unwrap();
        }

        let a = proc(&x, n);
        println!("{}", a.iter().map(|num| num.to_string()).collect::<Vec<String>>().join(" "));
    }
}