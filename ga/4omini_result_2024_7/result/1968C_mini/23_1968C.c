use std::io;

const N: usize = 500;
const A: u32 = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let n: usize = input.trim().parse().unwrap();
    let mut x = vec![0; N];
    let mut a = vec![0; N];

    // Reading values into the array x
    let mut input_values = String::new();
    io::stdin().read_line(&mut input_values).unwrap();
    
    let values: Vec<u32> = input_values
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for i in 1..n {
        x[i] = values[i - 1];
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
    println!("");
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        proc();
    }
}