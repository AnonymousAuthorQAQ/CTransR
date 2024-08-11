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
        let mut values = input.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let (n, k, a, b) = (values.next().unwrap(), values.next().unwrap(), values.next().unwrap(), values.next().unwrap());
        let m = min(n, k);
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        for x in 1..=n {
            arr[x] = values.next().unwrap();
        }
        for x in 1..=n {
            aa[x] = values.next().unwrap();
        }

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for x in 1..=m {
            p1 = max(p1, p1_ + aa[a] * (k - x + 1));
            p1_ += aa[a];
            a = arr[a];
            p2 = max(p2, p2_ + aa[b] * (k - x + 1));
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