use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<u32> = input.trim().split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut x: Vec<(u32, usize)> = vec![(0, 0)];
    for i in 0..n {
        x.push((x[i].0 ^ arr[i], i+1));
    }
    let mut x2: Vec<u32> = x.iter().map(|&(a, _)| a).collect();
    x.sort();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l-1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by_key(&(c ^ x2[l-1]), |&(a, _)| a).unwrap_or_else(|x| x);
        if i1 == n+1 || x[i1].1 > r || x[i1].0 != c ^ x2[l-1] {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search_by_key(&x2[l-1], |&(a, i)| a).unwrap_or_else(|x| x);
        if i2 == n+1 || x[i2].1 > r || x[i2].0 != x2[l-1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}