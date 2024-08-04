use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    io::stdin().read_line(&mut input).unwrap();
    for s in input.trim().split_whitespace() {
        arr.push(s.parse().unwrap());
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
        let mut parts = input.trim().split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = match x.binary_search_by_key(&c ^ x2[l - 1], |&(key, _)| key) {
            Ok(i) => i,
            Err(i) => i,
        };
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = match x.binary_search_by_key(&x2[l - 1], |&(key, _)| key) {
            Ok(i) => i,
            Err(i) => i,
        };
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}