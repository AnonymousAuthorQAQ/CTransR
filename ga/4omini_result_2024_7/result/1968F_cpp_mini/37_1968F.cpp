use std::io::{self, Write};
use std::cmp::Ordering;

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Pair(usize, usize);

fn solve() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = Vec::with_capacity(n);
    input.clear();
    stdin.read_line(&mut input).unwrap();
    for a in input.split_whitespace() {
        arr.push(a.parse::<usize>().unwrap());
    }

    let mut x = vec![Pair(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = Pair(x[i - 1].0 ^ arr[i - 1], i);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort();

    let mut results = Vec::new();

    for _ in 0..q {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            results.push("YES".to_string());
            continue;
        }

        let mut i1 = x.binary_search(&Pair(c ^ x2[l - 1], l)).unwrap_err();
        
        if i1 >= n || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            results.push("NO".to_string());
            continue;
        }
        
        let mut i2 = x.binary_search(&Pair(x2[l - 1], x[i1].1 + 1)).unwrap_err();
        
        if i2 >= n || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            results.push("NO".to_string());
        } else {
            results.push("YES".to_string());
        }
    }

    println!("{}", results.join("\n"));
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