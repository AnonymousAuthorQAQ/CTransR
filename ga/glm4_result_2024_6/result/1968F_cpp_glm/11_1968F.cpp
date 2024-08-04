use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

fn solve(n: usize, q: usize, arr: Vec<usize>) {
    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
        x2[i] = x[i].0;
    }
    
    x.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    let mut stdin = io::stdin();
    for _ in 0..q {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|probe| {
            if probe.0 < c ^ x2[l - 1] {
                Ordering::Less
            } else if probe.0 > c ^ x2[l - 1] {
                Ordering::Greater
            } else if probe.1 < l {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|probe| {
            if probe.0 < x2[l - 1] {
                Ordering::Less
            } else if probe.0 > x2[l - 1] {
                Ordering::Greater
            } else if probe.1 < x[i1].1 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let arr: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

        solve(n, q, arr);
    }
}