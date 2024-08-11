use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve<R: BufRead>(lines: &mut R) {
    let n_q: Vec<usize> = lines
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let n = n_q[0];
    let q = n_q[1];

    let arr: Vec<i32> = lines
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut x: Vec<(i32, usize)> = vec![(0, 0); n + 1];
    let mut x2: Vec<i32> = vec![0; n + 1];
    x[0] = (0, 0);

    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort_by(|a, b| a.cmp(b));

    for _ in 0..q {
        let l_r: Vec<usize> = lines
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let l = l_r[0];
        let r = l_r[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = match x.binary_search(&(c ^ x2[l - 1], l)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let mut i2 = match x.binary_search(&(x2[l - 1], x[i1].1 + 1)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}