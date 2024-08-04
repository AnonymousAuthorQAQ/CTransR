use std::io::{self, Read};

fn proc() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    let mut x = vec![0; n];
    for i in 1..n {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        x[i] = buffer.trim().parse().unwrap();
    }

    let mut a = vec![0; n];
    a[0] = x[1] + 1;

    for i in 1..n {
        let mut j = 0;
        while j <= 1_000_000_000 {
            a[i] = x[i] + j * a[i - 1];
            if (i + 1 < n && x[i + 1] < a[i]) || (i + 1 == n && a[i] != a[i - 1]) {
                break;
            }
            j += 1;
        }
    }

    for i in a {
        print!("{} ", i);
    }
    println!();
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}