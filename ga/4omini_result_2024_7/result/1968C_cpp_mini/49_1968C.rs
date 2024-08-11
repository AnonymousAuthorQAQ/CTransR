use std::io;

const N: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut x = [0; N];
    let mut a = [0; N];

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    
    for i in 1..n {
        let num: usize = split.next().unwrap().parse().unwrap();
        x[i] = num;
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) && (x[i + 1] < a[i]) && (a[i] != a[i - 1]) {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}