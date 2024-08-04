use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Pair {
    fs: u64,
    sc: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.fs.cmp(&other.fs)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Pair {
    fn new(fs: u64, sc: usize) -> Self {
        Pair { fs, sc }
    }
}

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.trim().split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut arr = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        arr.push(input.trim().parse().unwrap());
    }

    let mut x = Vec::new();
    x.push(Pair::new(0, 0));
    for i in 1..=n {
        x.push(Pair::new(x[i - 1].fs ^ arr[i - 1] as u64, i));
    }

    let mut x2 = Vec::with_capacity(n + 1);
    for i in 0..=n {
        x2.push(x[i].fs);
    }

    x.sort();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|p| p.fs.cmp(&c ^ x2[l - 1])) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n + 1 || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|p| p.fs.cmp(&x2[l - 1])) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n + 1 || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t: i32 = read_line().trim().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}