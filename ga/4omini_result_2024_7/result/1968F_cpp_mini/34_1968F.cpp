use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve(lines: &mut std::io::Lines<impl BufRead>) {
    let line = lines.next().unwrap().unwrap();
    let mut parts = line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let arr: Vec<i64> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i].0 = x[i - 1].0 ^ arr[i - 1];
        x[i].1 = i as i64;
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    x.sort();
    
    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let mut i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(fs, _)| fs).unwrap_err();
        
        if i1 == n + 1 || x[i1].1 > r as i64 || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let mut i2 = x.binary_search_by_key(&x2[l - 1], |&(fs, _)| fs).unwrap_err();
        
        if i2 == n + 1 || x[i2].1 > r as i64 || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}