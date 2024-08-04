use std::io::{self, Read};

const N: usize = 500;
const A: i64 = 1000000000;

fn proc() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    
    let mut x = [0; N];
    for i in 1..n {
        buffer.clear();
        io::stdin().read_line(&mut buffer).unwrap();
        x[i] = buffer.trim().parse().unwrap();
    }
    
    let mut a = [0; N];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && 1 < a[i]) {
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
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}