use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.trim().split_whitespace();
    let n: usize = tokens.next().unwrap().parse().unwrap();
    let q: usize = tokens.next().unwrap().parse().unwrap();

    let mut arr = Vec::with_capacity(n);
    io::stdin().read_line(&mut input).unwrap();
    let mut tokens = input.trim().split_whitespace();
    for _ in 0..n {
        arr.push(tokens.next().unwrap().parse().unwrap());
    }

    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    let mut x2 = x.clone();
    x2.sort_unstable();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let l: usize = tokens.next().unwrap().parse().unwrap();
        let r: usize = tokens.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x2.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|i| i);

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x2.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|i| i);

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut line_iter = stdin.lock().lines();

    let t: usize = line_iter.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}