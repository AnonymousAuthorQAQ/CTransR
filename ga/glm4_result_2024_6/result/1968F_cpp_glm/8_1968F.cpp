use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut parts = line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(stdin.lock().lines().next().unwrap().unwrap().parse().unwrap());
    }

    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for (i, &num) in arr.iter().enumerate() {
        x.push((x[i].0 ^ num, i + 1));
    }
    let mut x2 = x.clone();

    x.sort_by(|a, b| {
        if a.0 == b.0 {
            Ordering::Equal
        } else if a.0 < b.0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    for _ in 0..q {
        let line = stdin.lock().lines().next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by(|v| {
            if v.0 == c ^ x2[l - 1] {
                Ordering::Equal
            } else if v.0 < c ^ x2[l - 1] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).unwrap_err();

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by(|v| {
            if v.0 == x2[l - 1] {
                Ordering::Equal
            } else if v.0 < x2[l - 1] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).unwrap_err();

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t: usize = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}