use std::io::{self, BufRead};

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap();
        let mut values = line.trim().split_whitespace().map(|x| x.parse().unwrap());
        let n: usize = values.next().unwrap();
        let k: usize = values.next().unwrap();
        let a: usize = values.next().unwrap();
        let b: usize = values.next().unwrap();

        let m = min(n, k);
        let (mut p1, mut p1_, mut p2, mut p2_) = (0i64, 0i64, 0i64, 0i64);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);
        let (mut a_, mut b_) = (a, b);

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        let line = lines.next().unwrap();
        let values = line.trim().split_whitespace().map(|x| x.parse().unwrap());
        for x in 1..=n {
            arr[x] = values.clone().next().unwrap();
        }

        let line = lines.next().unwrap();
        let values = line.trim().split_whitespace().map(|x| x.parse().unwrap());
        for x in 1..=n {
            aa[x] = values.clone().next().unwrap();
        }

        for x in 1..=m {
            p1 = maxl(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = maxl(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
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