use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    let n: usize = read_line().trim().parse().unwrap();
    for i in 1..n {
        x[i] = read_line().trim().parse().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!();
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn main() {
    let t: usize = read_line().trim().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}