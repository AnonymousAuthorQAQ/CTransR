use std::io::{self, BufRead};
use std::vec::Vec;
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());
    let first_line = lines.next().unwrap();
    let mut params = first_line.split_whitespace();
    let n: usize = params.next().unwrap().parse().unwrap();
    let q: usize = params.next().unwrap().parse().unwrap();
    
    let mut arr = vec![0; n];
    let second_line = lines.next().unwrap();
    let arr_vec: Vec<i32> = second_line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap()).collect();
    
    for i in 0..n {
        arr[i] = arr_vec[i];
    }
    
    let mut x: Vec<(i32, usize)> = vec![(0, 0); n + 1];
    let mut x2: Vec<i32> = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    x.sort_by(|a, b| a.cmp(b));

    for _ in 0..q {
        let line = lines.next().unwrap();
        let mut lr = line.split_whitespace();
        let l: usize = lr.next().unwrap().parse().unwrap();
        let r: usize = lr.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
            continue;
        }
        
        // Find the lower bound for c ^ x2[l - 1]
        let target = c ^ x2[l - 1];
        let mut i1 = x.binary_search_by_key(&(target, l), |&(v, idx)| (v, idx)).unwrap_err();
        
        if i1 == x.len() || x[i1].1 > r || x[i1].0 != target {
            println!("NO");
            continue;
        }
        
        // Find the lower bound for x2[l - 1]
        let mut i2 = x.binary_search_by_key(&(x2[l - 1], x[i1].1 + 1), |&(v, idx)| (v, idx)).unwrap_err();
        
        if i2 == x.len() || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines = reader.lines().map(|line| line.unwrap());
    let mut line_iter = lines.iter();
    
    let t: usize = line_iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}