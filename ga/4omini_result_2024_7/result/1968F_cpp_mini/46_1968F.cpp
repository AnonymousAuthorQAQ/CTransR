use std::io::{self, Read};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Pii(usize, usize);

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

fn solve(iter: &mut std::str::SplitWhitespace) {
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    
    let mut arr: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(iter.next().unwrap().parse().unwrap());
    }

    let mut x = vec![(0, 0); n + 1]; // (xor up to the index, index)
    let mut x2 = vec![0; n + 1]; // prefix xor array
    x[0] = (0, 0);
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
        x2[i] = x[i].0;
    }
    
    // Sort the pairs
    x.sort_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
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

        let target = (c ^ x2[l - 1], l);
        let i1 = x.binary_search_by(|&(val, _)| {
            if val < target.0 {
                Ordering::Less
            } else if val > target.0 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }).unwrap_or_else(|x| x);

        // Check conditions for i1
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by(|&(val, _)| {
            if val < x2[l - 1] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).unwrap_or_else(|x| x);

        // Check conditions for i2
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}