use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());
    
    let first_line = input.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let arr: Vec<i64> = input.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    x.sort_unstable_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    
    for _ in 0..q {
        let line = input.next().unwrap();
        let mut parts = line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|x| x);
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().map(|line| line.unwrap());
    let t: usize = input.clone().next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}