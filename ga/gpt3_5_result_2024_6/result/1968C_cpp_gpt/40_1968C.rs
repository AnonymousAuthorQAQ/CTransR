use std::io;

const N: usize = 500;
const X: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let x: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut a: Vec<usize> = vec![0; n];
    a[0] = x[0] + 1;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i] || i + 1 >= n) && a[i] != a[i - 1] {
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

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}