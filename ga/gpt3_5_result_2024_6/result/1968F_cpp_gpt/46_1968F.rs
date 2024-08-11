use std::io::{self, Read};
use std::cmp::Ordering;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace()
                             .map(|x| x.parse().unwrap())
                             .collect();
    
    let mut x: Vec<(i32, usize)> = vec![(0, 0); n+1];
    for i in 1..=n {
        x[i] = (x[i-1].0 ^ arr[i-1], i);
    }
    let mut x2: Vec<i32> = x.iter().map(|&(a, _)| a).collect();
    x.sort();
    
    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l-1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        match x.binary_search_by(|&(val, _)| val.cmp(&(c ^ x2[l-1]))) {
            Ok(i1) => {
                if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l-1]) {
                    println!("NO");
                    continue;
                }
                match x.binary_search_by(|&(val, i)| {
                    if i < x[i1].1 + 1 {
                        Ordering::Less
                    } else {
                        val.cmp(&x2[l-1])
                    }
                }) {
                    Ok(i2) => {
                        if i2 == n+1 || x[i2].1 > r || x[i2].0 != x2[l-1] {
                            println!("NO");
                        } else {
                            println!("YES");
                        }
                    }
                    Err(_) => {
                        println!("NO");
                    }
                }
            }
            Err(_) => {
                println!("NO");
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}