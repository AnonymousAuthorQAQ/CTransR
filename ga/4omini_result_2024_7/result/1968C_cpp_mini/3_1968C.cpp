use std::io;

const N: usize = 500;
const A: usize = 1000000000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse");

    let mut x = vec![0; N];
    let mut a = vec![0; N];

    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let values: Vec<usize> = input.trim().split_whitespace()
                                   .map(|s| s.parse().expect("Failed to parse"))
                                   .collect();

    for i in 1..n {
        x[i] = values[i - 1]; // Reading values into x
    }

    a[0] = x[1] + 1; // equivalent to a[0] = x[1] + 1 in C++

    for i in 1..n {
        for j in 0..=A {
            a[i] = x[i] + j * a[i - 1];
            if (if i + 1 < n { x[i + 1] } else { 1 }) < a[i] && a[i] != a[i - 1] {
                break;
            }
        }
    }

    for i in 0..n {
        print!("{} ", a[i]);
    }
    println!(); // Use println! for new line after printing the array.
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse");

    for _ in 0..t {
        proc();
    }
}