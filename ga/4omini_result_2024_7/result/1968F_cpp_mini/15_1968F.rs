use std::io;
use std::cmp::Ordering;
use std::collections::VecDeque;

fn solve() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (n, q) = (parts[0], parts[1]);

    let mut arr = Vec::new();
    input.clear();
    stdin.read_line(&mut input).unwrap();
    arr.extend(input.split_whitespace().map(|x| x.parse::<i64>().unwrap()));

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i].0 = x[i - 1].0 ^ arr[i - 1];
        x[i].1 = i as i64;
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort_by_key(|&pair| pair.0);

    for _ in 0..q {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let lr: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (l, r) = (lr[0], lr[1]);

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let c_pair = (c ^ x2[l - 1], l as i64);
        let i1 = match x.binary_search_by(|pair| pair.0.cmp(&c_pair.0)) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        if i1 == n + 1 || x[i1].1 > r as i64 || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|pair| pair.0.cmp(&(x2[l - 1], x[i1].1 + 1))) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        if i2 == n + 1 || x[i2].1 > r as i64 || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}