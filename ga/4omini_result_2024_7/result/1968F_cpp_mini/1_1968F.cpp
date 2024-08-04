use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut reader = handle.lines().map(|l| l.unwrap());
    let t: usize = reader.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve(&mut reader);
    }
}

fn solve<R: BufRead>(reader: &mut R) {
    let first_line = reader.lines().next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut arr: Vec<i64> = reader.lines()
        .take(n)
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort();

    for _ in 0..q {
        let query = reader.lines().next().unwrap().unwrap();
        let mut query_parts = query.split_whitespace();
        let l: usize = query_parts.next().unwrap().parse().unwrap();
        let r: usize = query_parts.next().unwrap().parse().unwrap();

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