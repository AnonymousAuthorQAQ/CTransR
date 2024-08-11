use std::io::{self, Read};
use std::collections::HashMap;

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().split('\n');
    let mut get_line = || -> Vec<i32> {
        lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect()
    };
    
    let t = get_line()[0];
    for _ in 0..t {
        let nm: Vec<i32> = get_line();
        let n = nm[0] as usize;
        let q = nm[1];
        
        let mut input_arr = String::new();
        io::stdin().read_line(&mut input_arr).unwrap();
        let arr: Vec<i32> = input_arr.split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        
        let mut x: Vec<(i32, usize)> = vec![(0, 0)];
        arr.iter().enumerate().for_each(|(i, &val)| {
            let xor_val = x[i].0 ^ val;
            x.push((xor_val, i+1));
        });
        
        x.sort();
        
        for _ in 0..q {
            let queries: Vec<i32> = get_line();
            let l = queries[0] as usize;
            let r = queries[1] as usize;
            
            let c = x[r].0 ^ x[l-1].0;
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let i1 = match x.binary_search_by_key(&(c ^ x[l-1].0), |&val| val.0) {
                Ok(val) => val,
                Err(_e) => {
                    println!("NO");
                    continue;
                }
            };
            
            if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x[l-1].0) {
                println!("NO");
                continue;
            }
            
            let i2 = match x.binary_search_by_key(&x[l-1].0, |&val| val.0) {
                Ok(val) => val,
                Err(_e) => {
                    println!("NO");
                    continue;
                }
            };
            
            if i2 == n+1 || x[i2].1 > r || x[i2].0 != x[l-1].0 {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

fn main() {
    solve();
}