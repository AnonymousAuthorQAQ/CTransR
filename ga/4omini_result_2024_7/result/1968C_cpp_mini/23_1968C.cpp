use std::io::{self, BufRead};

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc(x: &mut Vec<usize>, n: usize) -> Vec<usize> {
    let mut a = vec![0; n];
    
    a[0] = x[1] + 1;
    
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
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Read number of test cases
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Read number of elements
        let mut x = vec![0; n];
        
        for i in 1..n {
            x[i] = lines.next().unwrap().unwrap().parse().unwrap(); // Read elements into x
        }
        
        let a = proc(&mut x, n);
        println!("{}", a.iter().map(|&val| val.to_string()).collect::<Vec<String>>().join(" "));
    }
}