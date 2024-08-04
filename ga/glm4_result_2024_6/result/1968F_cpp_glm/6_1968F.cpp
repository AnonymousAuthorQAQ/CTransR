use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn solve() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(stdin.next().unwrap().parse().unwrap());
    }

    let mut x = Vec::with_capacity(n + 1);
    let mut x2 = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    for val in x.iter() {
        x2.push(val.0);
    }
    x.sort();

    for _ in 0..q {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by(|&(val, idx)| (val ^ x2[l - 1], idx)).unwrap();
        if i1 == n || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by(|&(val, idx)| (val, idx)).unwrap();
        if i2 == n || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}