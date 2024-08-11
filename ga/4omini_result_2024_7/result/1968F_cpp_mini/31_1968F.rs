use std::io::{self, Read, Write};
use std::cmp::{self, Ord};
use std::collections::BinaryHeap;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Pair {
    x: i64,
    index: usize,
}

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iterator = input.split_whitespace();

    let n: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();
    
    let mut arr = vec![0; n];
    for i in 0..n {
        arr[i] = iterator.next().unwrap().parse().unwrap();
    }
    
    let mut x: Vec<Pair> = vec![(0, 0).into(); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = Pair { x: 0, index: 0 };
    
    for i in 1..=n {
        x[i] = Pair {
            x: x[i - 1].x ^ arr[i - 1],
            index: i,
        };
    }
    
    for i in 0..=n {
        x2[i] = x[i].x;
    }
    
    x.sort(); // Sort by the x value and then by index due to tuple ordering
    
    let mut output = String::new();
    
    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            output.push_str("YES\n");
            continue;
        }
        
        let mut i1 = x.binary_search(&Pair { x: c ^ x2[l - 1], index: l }).unwrap_err();
        
        if i1 == n + 1 || x[i1].index > r || x[i1].x != (c ^ x2[l - 1]) {
            output.push_str("NO\n");
            continue;
        }
        
        let mut i2 = x.binary_search(&Pair { x: x2[l - 1], index: x[i1].index + 1 }).unwrap_err();
        
        if i2 == n + 1 || x[i2].index > r || x[i2].x != x2[l - 1] {
            output.push_str("NO\n");
        } else {
            output.push_str("YES\n");
        }
    }
    
    print!("{}", output);
}

fn main() {
    let t: usize = {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        input.split_whitespace().next().unwrap().parse().unwrap()
    };
    
    for _ in 0..t {
        solve();
    }
}

impl From<(i64, usize)> for Pair {
    fn from(tuple: (i64, usize)) -> Self {
        Pair { x: tuple.0, index: tuple.1 }
    }
}