use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let (n, q) = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
    let arr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for (i, &num) in arr.iter().enumerate() {
        x.push((x[i].0 ^ num, i + 1));
    }
    
    let mut x2 = Vec::with_capacity(n + 1);
    for &(val, _) in &x {
        x2.push(val);
    }
    
    x.sort_unstable();
    
    for _ in 0..q {
        let (l, r) = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let mut heap = BinaryHeap::new();
        for i in l - 1..n {
            heap.push((x[i].0 ^ c, x[i].1));
        }
        
        let mut found = false;
        while let Some((val, idx)) = heap.pop() {
            if idx <= r && val == c ^ x2[l - 1] {
                found = true;
                break;
            }
        }
        
        if !found {
            println!("NO");
            continue;
        }
        
        let mut heap = BinaryHeap::new();
        for i in l - 1..n {
            heap.push((x[i].0, x[i].1));
        }
        
        let mut found = false;
        while let Some((val, idx)) = heap.pop() {
            if idx <= r && val == x2[l - 1] {
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
    let t = 1; // read from input or set to 1 for testing
    for _ in 0..t {
        solve();
    }
}