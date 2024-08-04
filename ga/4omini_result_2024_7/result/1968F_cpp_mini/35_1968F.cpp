use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let (n, q): (usize, usize) = {
        let parts: Vec<usize> = first_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        (parts[0], parts[1])
    };

    let arr: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1] as i64, i);
        x2[i] = x[i].0;
    }
    
    // Sort `x`
    let mut sorted_x = x.clone();
    sorted_x.sort_unstable_by(|a, b| {
        if a.0 == b.0 {
            a.1.cmp(&b.1)
        } else {
            a.0.cmp(&b.0)
        }
    });
    
    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let (l, r): (usize, usize) = {
            let parts: Vec<usize> = query_line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            (parts[0], parts[1])
        };

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = sorted_x.binary_search_by(|&p| {
            let cmp_val = c ^ x2[l - 1];
            if p.0 < cmp_val {
                Ordering::Less
            } else if p.0 > cmp_val {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let i1 = match i1 {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        if i1 == n + 1 || sorted_x[i1].1 > r || sorted_x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let mut i2 = sorted_x.binary_search_by(|&p| {
            if p.0 < x2[l - 1] {
                Ordering::Less
            } else if p.0 > x2[l - 1] {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        let i2 = match i2 {
            Ok(idx) => idx,
            Err(idx) => idx,
        };

        if i2 == n + 1 || sorted_x[i2].1 > r || sorted_x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}


fn main() {
    let stdin = io::stdin();
    let t: usize = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}