use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n: usize = n.trim().parse().unwrap();
    
    let mut x = vec![0; N];
    for i in 1..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();
    }

    let mut a = vec![0; N];
    a[0] = x[1] + 1;
    
    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) && (x[i + 1] < a[i] && a[i] != a[i - 1]) {
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
    let mut t = String::new();
    io::stdin().read_line(&mut t).unwrap();
    let t: usize = t.trim().parse().unwrap();
    
    for _ in 0..t {
        proc();
    }
}