use std::io;
use std::cmp::{min, max};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut values = line.trim().split_whitespace().map(|x| x.parse().unwrap());
        let (n, k, a, b) = (values.next().unwrap(), values.next().unwrap(), values.next().unwrap(), values.next().unwrap());
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        line.clear();
        stdin.read_line(&mut line).unwrap();
        for (i, x) in line.trim().split_whitespace().map(|x| x.parse().unwrap()).enumerate() {
            arr[i + 1] = x;
        }

        line.clear();
        stdin.read_line(&mut line).unwrap();
        for (i, x) in line.trim().split_whitespace().map(|x| x.parse().unwrap()).enumerate() {
            aa[i + 1] = x;
        }

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for x in 1..=min(n, k) {
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