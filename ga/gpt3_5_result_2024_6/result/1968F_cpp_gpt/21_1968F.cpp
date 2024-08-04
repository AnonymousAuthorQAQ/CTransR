use std::io::{self, Read};
use std::collections::HashMap;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect();

    let mut x: Vec<(i32, usize)> = Vec::new();
    x.push((0, 0));
    let mut x2: Vec<i32> = vec![0; n+1];
    for i in 0..n {
        x.push((x[i].0 ^ arr[i], i+1));
        x2[i+1] = x[i+1].0;
    }
    x.sort();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l-1];
        if c == 0 {
            println!("YES");
            continue;
        }
        if let Some(i1) = x.iter().position(|&(_, idx)| idx >= l && x[idx].0 == c ^ x2[l-1]) {
            if x[i1].1 > r || x[i1].0 != c ^ x2[l-1] {
                println!("NO");
                continue;
            }
            
            if let Some(i2) = x.iter().position(|&(_, idx)| idx > i1 && x[idx].1 > r && x[idx].0 == x2[l-1]) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            println!("NO");
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