use std::io;

const N: usize = 500;
const A: i32 = 1_000_000_000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Read the number of elements
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Please provide a number");

    // Read the elements into x
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    for (i, word) in input.trim().split_whitespace().enumerate() {
        if i < n - 1 {
            x[i + 1] = word.parse().expect("Please provide a valid number");
        }
    }

    a[0] = x[1] + 1;
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
    println!();
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please provide a valid number");
    
    for _ in 0..t {
        proc();
    }
}