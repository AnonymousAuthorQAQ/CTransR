use std::io;
use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().expect("Failed to parse n");
    let q: usize = iter.next().unwrap().parse().expect("Failed to parse q");

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        arr.push(input.trim().parse().expect("Failed to parse array element"));
    }

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];

    x[0] = (0, 0);
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    x.sort_by(|a, b| a.0.cmp(&b.0));

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().expect("Failed to parse l");
        let r: usize = iter.next().unwrap().parse().expect("Failed to parse r");

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by(|v| v.0.cmp(&c ^ x2[l - 1])).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by(|v| v.0.cmp(&x2[l - 1])).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse t");

    for _ in 0..t {
        solve();
    }
}