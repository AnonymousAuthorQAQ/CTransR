use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and q
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    // Read the array
    let mut arr: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut x: Vec<(i64, usize)> = vec![(0, 0); n + 1];
    let mut x2: Vec<i64> = vec![0; n + 1];

    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    // Sort x
    x.sort_by(|a, b| {
        if a.0 != b.0 {
            a.0.cmp(&b.0)
        } else {
            a.1.cmp(&b.1)
        }
    });

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut parts = query_line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_err();
        if i1 == x.len() || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_err();
        if i2 == x.len() || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let first_line = reader.lines().next().unwrap().unwrap();
    let t: usize = first_line.parse().unwrap();

    for _ in 0..t {
        solve();
    }
}