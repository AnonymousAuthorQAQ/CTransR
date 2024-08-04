use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();
        arr.push(x);
    }
    
    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    
    let mut x2 = Vec::with_capacity(n + 1);
    for i in 0..=n {
        x2.push(x[i].0);
    }
    
    x.sort();
    
    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let mut heap = BinaryHeap::new();
        for i in l..=r {
            heap.push((x[i].0 ^ c, x[i].1));
        }
        
        let mut found = false;
        while let Some((val, idx)) = heap.pop() {
            if val == x2[l - 1] {
                found = true;
                break;
            }
        }
        
        if found {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn main() {
    let t: i32 = io::stdin().read_line().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}