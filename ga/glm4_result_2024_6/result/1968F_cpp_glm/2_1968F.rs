use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nq: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = nq[0];
    let q = nq[1];

    let arr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for (i, &a) in arr.iter().enumerate() {
        x.push((x[i].0 ^ a, i + 1));
    }
    let mut x2 = Vec::with_capacity(n + 1);
    for &(_, idx) in &x {
        x2.push(x[idx].0);
    }
    x2.sort();

    for _ in 0..q {
        let lr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let l = lr[0];
        let r = lr[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x2.binary_search(&c ^ x2[l - 1]) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x2.binary_search(&x2[l - 1]) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t = 1; // Read from input if needed
    for _ in 0..t {
        solve();
    }
}