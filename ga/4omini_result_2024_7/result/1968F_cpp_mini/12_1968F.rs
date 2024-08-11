use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    let first_line = input.next().unwrap().unwrap();
    
    let mut n_q = first_line.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
        
    let n = n_q[0];
    let q = n_q[1];

    let mut arr = vec![0; n];
    
    let second_line = input.next().unwrap().unwrap();
    for (i, val) in second_line.split_whitespace().enumerate() {
        arr[i] = val.parse::<i64>().unwrap();
    }
    
    let mut x = vec![(0, 0); n+1];
    let mut x2 = vec![0; n+1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i-1].0 ^ arr[i-1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    x.sort_by(|a, b| a.cmp(b));

    for _ in 0..q {
        let query_line = input.next().unwrap().unwrap();
        let mut l_r = query_line.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        
        let l = l_r[0];
        let r = l_r[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_err();
        
        if i1 >= n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_err();
        
        if i2 >= n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let t = stdin.lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    
    for _ in 0..t {
        solve();
    }
}