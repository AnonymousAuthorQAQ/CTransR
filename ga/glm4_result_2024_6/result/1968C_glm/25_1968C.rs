use std::io::{self, BufRead};

fn proc() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut x = vec![0; n];
    for i in 1..n {
        x[i] = lines.next().unwrap().unwrap().parse().unwrap();
    }

    let mut a = vec![0; n];
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
        }
    }

    for a_val in a {
        print!("{} ", a_val);
    }
    println!();
}

fn main() {
    let t: i32 = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        proc();
    }
}