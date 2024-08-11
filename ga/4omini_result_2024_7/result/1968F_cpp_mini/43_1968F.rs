use std::io::{self, BufRead};
use std::cmp::{self, Ord};
use std::vec::Vec;

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    first: i64,
    second: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        if self.first == other.first {
            self.second.cmp(&other.second)
        } else {
            self.first.cmp(&other.first)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn solve() {
    let stdin = io::stdin();
    let mut buf = String::new();
    
    stdin.lock().read_line(&mut buf).unwrap();
    let mut parts = buf.split_whitespace();
    
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::new();
    buf.clear();
    stdin.lock().read_line(&mut buf).unwrap();
    arr.extend(buf.split_whitespace().map(|x| x.parse::<i64>().unwrap()));

    let mut x: Vec<Pair> = vec![Pair { first: 0, second: 0 }; n + 1];
    let mut x2: Vec<i64> = vec![0; n + 1];

    for i in 1..=n {
        x[i].first = x[i - 1].first ^ arr[i - 1];
        x[i].second = i;
    }

    for i in 0..=n {
        x2[i] = x[i].first;
    }

    x.sort(); // Sorting pairs

    for _ in 0..q {
        buf.clear();
        stdin.lock().read_line(&mut buf).unwrap();
        let mut query_parts = buf.split_whitespace();
        
        let l: usize = query_parts.next().unwrap().parse().unwrap();
        let r: usize = query_parts.next().unwrap().parse().unwrap();
        
        let c: i64 = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search(&Pair { first: c ^ x2[l - 1], second: l }).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].second > r || x[i1].first != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search(&Pair { first: x2[l - 1], second: x[i1].second + 1 }).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].second > r || x[i2].first != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut buf = String::new();
    
    stdin.lock().read_line(&mut buf).unwrap();
    let t: usize = buf.trim().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}