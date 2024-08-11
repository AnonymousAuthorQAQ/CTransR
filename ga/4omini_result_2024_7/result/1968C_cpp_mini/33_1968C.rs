use std::io::{self, BufRead};

const N: usize = 500; 
const A: i32 = 1_000_000_000;

fn proc(n: usize, x: &[i32]) {
    let mut a = vec![0; N]; 
    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            let next_x = if (i + 1) < n { x[i + 1] } else { 1 };
            if next_x < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
        let mut x = vec![0; N];
        
        for i in 1..n {
            x[i] = lines.next().unwrap().unwrap().parse().unwrap();
        }

        proc(n, &x);
    }
}