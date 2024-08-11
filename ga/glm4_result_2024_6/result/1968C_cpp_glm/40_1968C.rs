use std::io;

const N: usize = 500;
const X: usize = 500;
const A: usize = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let mut x = vec![0; N];
    for i in 1..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();
    }

    let mut a = vec![0; N];
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if x.get(i + 1).unwrap_or(&1) < &a[i] && a[i] != a[i - 1] {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}