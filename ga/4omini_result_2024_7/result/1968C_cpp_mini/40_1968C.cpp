use std::io;

const N: usize = 500;
const A: i64 = 1_000_000_000;

fn proc() {
    let mut x = vec![0; N];
    let mut a = vec![0; N];
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Not a valid number");

    for i in 1..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        x[i] = input.trim().parse::<i64>().expect("Not a valid number");
    }

    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        loop {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (a[i] != a[i - 1] && i + 1 == n) {
                break;
            }
            j += 1;
            if j > A {
                break; // safety break similar to C++ code limit
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
    let t: usize = input.trim().parse().expect("Not a valid number");

    for _ in 0..t {
        proc();
    }
}