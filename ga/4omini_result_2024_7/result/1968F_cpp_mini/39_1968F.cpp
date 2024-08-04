use std::cmp::Ordering;
use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut arr = vec![0; n];
    let mut input = lines.next().unwrap().unwrap();
    let mut elements = input.split_whitespace();
    for i in 0..n {
        arr[i] = elements.next().unwrap().parse().unwrap();
    }

    let mut x: Vec<(i64, usize)> = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }
    x.sort_by(|a, b| a.0.cmp(&b.0));

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut lr = query_line.split_whitespace();
        let l: usize = lr.next().unwrap().parse().unwrap();
        let r: usize = lr.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let mut i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let mut i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|x| x);

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let first_line = handle.lines().next().unwrap().unwrap();
    let t: usize = first_line.trim().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}