use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    let second_line = lines.next().unwrap().unwrap();
    for num in second_line.split_whitespace() {
        arr.push(num.parse::<i64>().unwrap());
    }
    
    let mut x = vec![(0, 0); n+1];
    let mut x2 = vec![0; n+1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    // Sort x based on the first item of the tuple
    let mut x_sorted = x.clone();
    x_sorted.sort_by(|a, b| a.cmp(b));

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let mut i1 = x_sorted.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x_sorted[i1].1 > r || x_sorted[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let mut i2 = x_sorted.binary_search(&(x2[l - 1], x_sorted[i1].1 + 1)).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x_sorted[i2].1 > r || x_sorted[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let input_lines = stdin.lock().lines();

    let t: usize = input_lines.clone().nth(0).unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}