use std::io::{self, Read};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Pair {
    first: i64,
    second: usize,
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve(lines: &mut std::slice::Iter<'_, &str>) {
    let n_q: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let n = n_q[0];
    let q = n_q[1];

    let arr: Vec<i64> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    
    let mut x: Vec<Pair> = vec![Pair { first: 0, second: 0 }; n + 1];
    let mut x2: Vec<i64> = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = Pair { first: x[i - 1].first ^ arr[i - 1], second: i };
        x2[i] = x[i].first;
    }

    x.sort_unstable_by(|a, b| a.first.cmp(&b.first));

    for _ in 0..q {
        let l_r: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let l = l_r[0];
        let r = l_r[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = x.binary_search(&Pair { first: c ^ x2[l - 1], second: l }).unwrap_or_else(|e| e);
        
        if i1 == x.len() || x[i1].second > r || x[i1].first != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let mut i2 = x.binary_search(&Pair { first: x2[l - 1], second: x[i1].second + 1 }).unwrap_or_else(|e| e);
        
        if i2 == x.len() || x[i2].second > r || x[i2].first != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}