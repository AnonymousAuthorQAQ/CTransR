use std::io;
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    for _ in 0..t {
        solve(&stdin);
    }
}

fn solve(stdin: &io::Stdin) {
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr: Vec<i32> = Vec::new();
    input.clear();
    stdin.read_line(&mut input).unwrap();
    arr.extend(input.split_whitespace().map(|x| x.parse::<i32>().unwrap()));
    
    let mut x: Vec<(i32, usize)> = vec![(0, 0); n + 1];
    let mut x2: Vec<i32> = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    x.sort_by(|a, b| a.cmp(b));
    
    for _ in 0..q {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}