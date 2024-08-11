use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut values = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = values.next().unwrap();
    let q = values.next().unwrap();
    
    let mut arr = Vec::with_capacity(n as usize);
    for _ in 0..n {
        arr.push(values.next().unwrap());
    }
    
    let mut x = Vec::with_capacity(n as usize + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    
    let mut x2 = Vec::with_capacity(n as usize + 1);
    for i in 0..=n {
        x2.push(x[i].0);
    }
    
    x.sort_unstable();
    
    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let l = values.next().unwrap();
        let r = values.next().unwrap();
        
        let c = x2[r as usize] ^ x2[(l - 1) as usize];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search_by_key(&(c ^ x2[(l - 1) as usize]), |&(ref k, _)| *k).unwrap_or_else(|x| x);
        
        if i1 == n as usize || x[i1].1 > r || x[i1].0 != (c ^ x2[(l - 1) as usize]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search_by_key(&(x2[(l - 1) as usize], x[i1].1 + 1), |&(ref k, ref s)| (*k, *s)).unwrap_or_else(|x| x);
        
        if i2 == n as usize || x[i2].1 > r || x[i2].0 != x2[(l - 1) as usize] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t = {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        input.trim().parse::<i32>().unwrap()
    };

    for _ in 0..t {
        solve();
    }
}