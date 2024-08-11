use std::io::{self, Read};
use std::collections::HashMap;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split(" ").collect();
    let n: usize = parts[0].parse().unwrap();
    let q: usize = parts[1].parse().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(i32, usize)> = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 0..n {
        x.push((x[i].0 ^ arr[i], i + 1));
    }

    let mut x2: Vec<i32> = x.iter().map(|&a| a.0).collect();
    x.sort();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let query: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let l = query[0];
        let r = query[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&a| a.0).unwrap_or_else(|x| x);
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let mut i2 = x.binary_search_by_key(&x2[l - 1], |&a| a.0).unwrap_or_else(|x| x);
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}