use std::io::{self, BufRead};
use std::vec::Vec;

fn solve() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = vec![0; n];
    let second_line = lines.next().unwrap().unwrap();
    for (i, num) in second_line.split_whitespace().enumerate() {
        arr[i] = num.parse().unwrap();
    }
    
    // Prepare to compute the prefix xors
    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    // Sort the x array
    x.sort();
    
    while q > 0 {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
        } else {
            let i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|e| e);
            
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
            } else {
                let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|e| e);
                
                if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                    println!("NO");
                } else {
                    println!("YES");
                }
            }
        }
        
        q -= 1;
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines = reader.lines();
    
    let mut iter = lines.map(|line| line.unwrap());
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}