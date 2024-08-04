use std::io;

const N: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse input");
    let mut x = [0; N];
    for i in 1..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Failed to parse input");
    }
    let mut a = [0; N];
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
        println!("{}", a[i]);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse input");
    for _ in 0..t {
        proc();
    }
}