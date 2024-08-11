use std::io;
use std::cmp::min;
use std::cmp::max;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();

        let mut arr = [0; N + 1];
        let mut aa = [0; N + 1];

        for x in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[x] = input.trim().parse().unwrap();
        }

        for x in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa[x] = input.trim().parse().unwrap();
        }

        let m = min(n, k);
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