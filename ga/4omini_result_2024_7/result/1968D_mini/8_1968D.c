use std::io::{self, BufRead};

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (n, k, mut a, mut b) = (values[0], values[1], values[2], values[3]);
        let m = min(n, k);

        let mut arr: Vec<usize> = vec![0; n + 1];
        let mut aa: Vec<i64> = vec![0; n + 1];

        let arr_line = lines.next().unwrap().unwrap();
        for (i, val) in arr_line.split_whitespace().enumerate() {
            arr[i + 1] = val.parse().unwrap();
        }

        let aa_line = lines.next().unwrap().unwrap();
        for (i, val) in aa_line.split_whitespace().enumerate() {
            aa[i + 1] = val.parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, aa[b] * (k - x as i64 + 1) + p2_);
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