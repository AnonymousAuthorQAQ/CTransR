use std::io;
use std::io::{stdout, Write};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.trim().split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(0);
    }
    
    for i in 0..n {
        arr[i] = tokens.next().unwrap().parse().unwrap();
    }

    let mut x = Vec::with_capacity(n + 1);
    let mut x2 = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    
    for i in 0..=n {
        x2.push(x[i].0);
    }
    x2.sort();
    
    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let l: usize = tokens.next().unwrap().parse().unwrap();
        let r: usize = tokens.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = match x.binary_search_by_key(&(c ^ x2[l - 1]), |x| x.0) {
            Ok(i) => i,
            Err(i) => i,
        };
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = match x.binary_search_by_key(&x2[l - 1], |x| x.0) {
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
    let t: usize = read_int();
    for _ in 0..t {
        solve();
    }
}

fn read_int() -> usize {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}