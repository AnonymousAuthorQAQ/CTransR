use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let k: usize = line.next().unwrap().parse().unwrap();
        let a: usize = line.next().unwrap().parse().unwrap();
        let b: usize = line.next().unwrap().parse().unwrap();

        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse().unwrap();
        }
        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse().unwrap();
        }

        let m = std::cmp::min(n, k);
        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;
        for _ in 0..m {
            p1 = std::cmp::max(p1, p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];

            p2 = std::cmp::max(p2, p2_ + aa[b]);
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