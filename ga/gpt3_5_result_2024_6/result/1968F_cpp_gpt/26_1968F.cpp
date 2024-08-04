use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    
    let mut arr: Vec<i32> = Vec::new();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    arr = input.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
    
    let mut x: Vec<(i32, usize)> = Vec::with_capacity(n + 1);
    x.push((0, 0));
    
    for i in 0..n {
        x.push((x[i].0 ^ arr[i], i + 1));
    }
    
    let mut x2: Vec<i32> = x.iter().map(|&(val, _)| val).collect();
    x.sort();
    
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(val, _)| val)
                    .unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x2[l - 1] {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search_by_key(&x2[l - 1], |&(val, idx)| idx)
                    .unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}