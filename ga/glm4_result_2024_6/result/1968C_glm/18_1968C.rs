use std::io;

const N: usize = 500;
const X: usize = 500;
const A: i32 = 1_000_000_000;

fn proc() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input");

    let mut x = [0; N];
    for i in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse().expect("Invalid input");
    }

    let mut a = [0; N];
    a[0] = x[1] + 1;
    for i in 1..n {
        let mut j = 0;
        loop {
            a[i] = x[i] + j * a[i - 1];
            let next_x = if i + 1 < n { x[i + 1] } else { 1 };
            if next_x < a[i] && a[i] != a[i - 1] {
                break;
            }
            j += 1;
            if j > A as usize {
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
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        proc();
    }
}