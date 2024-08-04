use std::io;

const N: usize = 500;
const A: usize = 1_000_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        proc_case();
    }
}

fn proc_case() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().unwrap();

    let mut x: [usize; N] = [0; N];
    let mut a: [usize; N] = [0; N];

    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let x_values: Vec<&str> = input.trim().split_whitespace().collect();
    for i in 1..n {
        x[i] = x_values[i - 1].parse().unwrap();
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n) then (x[i + 1]) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!(); // print newline
}