use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Pair(usize, usize);

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve<I: Iterator<Item=io::Result<String>>>(lines: &mut I) {
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let arr: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut x: Vec<Pair> = vec![(0, 0).into()];
    let mut x2: Vec<usize> = vec![0; n + 1];

    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i).into());
        x2[i] = x[i].0;
    }
    
    x.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut parts = query_line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|x| x);
        if i1 == x.len() || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let mut i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|x| x);
        if i2 == x.len() || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

impl From<(usize, usize)> for Pair {
    fn from(tuple: (usize, usize)) -> Self {
        Pair(tuple.0, tuple.1)
    }
}