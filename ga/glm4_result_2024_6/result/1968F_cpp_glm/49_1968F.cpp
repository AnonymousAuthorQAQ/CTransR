use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        arr.push(line.trim().parse().unwrap());
    }

    let mut x = Vec::with_capacity(n + 1);
    let mut x2 = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    for i in 0..=n {
        x2.push(x[i].0);
    }
    x2.sort();

    for _ in 0..q {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|probe| probe.0.cmp(&c ^ x2[l - 1])) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|probe| probe.0.cmp(&x2[l - 1])) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}