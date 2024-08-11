use std::io;
use std::cmp::{min, max};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace().map(|s| s.parse::<usize>().expect("Invalid input"));
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let a = values.next().unwrap();
        let b = values.next().unwrap();

        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            arr[i] = input.trim().parse().expect("Invalid input");
        }

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            aa[i] = input.trim().parse().expect("Invalid input");
        }

        let m = min(n, k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..m {
            p1 = max(p1, p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];

            p2 = max(p2, p2_ + aa[b]);
            p2_ += aa[b];
            b = arr[b];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}