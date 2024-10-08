use std::io;

const N: usize = 500;
const A: i64 = 1000000000;

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
    a[0] = (x[1] + 1) as i64;
    for i in 1..n {
        let mut j = 0;
        while j <= A {
            a[i] = x[i] as i64 + j * a[i - 1];
            if (i + 1 < n) && (a[i] < x[i + 1] as i64) || (i == n - 1) && (a[i] < 1) {
                break;
            }
            j += 1;
        }
    }

    for &ai in &a {
        print!("{} ", ai);
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