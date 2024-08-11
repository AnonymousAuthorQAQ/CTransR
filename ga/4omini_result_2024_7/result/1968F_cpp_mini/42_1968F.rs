use std::cmp::Ordering;
use std::io::{self, Read};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
struct Pair {
    first: i64,
    second: usize,
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve(&mut iter);
    }
}

fn solve<I>(iter: &mut I)
where
    I: Iterator<Item = &'static str>,
{
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(iter.next().unwrap().parse::<i64>().unwrap());
    }
    
    let mut x = vec![Pair { first: 0, second: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i].first = x[i - 1].first ^ arr[i - 1];
        x[i].second = i;
    }
    
    for i in 0..=n {
        x2[i] = x[i].first;
    }
    
    x.sort_by(|a, b| {
        if a.first == b.first {
            a.second.cmp(&b.second)
        } else {
            a.first.cmp(&b.first)
        }
    });

    for _ in 0..q {
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search(&Pair { first: c ^ x2[l - 1], second: l }).unwrap_err();
        if i1 == n + 1 || x[i1].second > r || x[i1].first != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search(&Pair { first: x2[l - 1], second: x[i1].second + 1 }).unwrap_err();
        if i2 == n + 1 || x[i2].second > r || x[i2].first != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}