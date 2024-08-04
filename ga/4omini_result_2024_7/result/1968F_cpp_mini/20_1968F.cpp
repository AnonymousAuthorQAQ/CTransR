use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Pair {
    fs: i64,
    sc: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.fs.cmp(&other.fs) {
            Ordering::Equal => self.sc.cmp(&other.sc),
            other => other,
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.fs == other.fs && self.sc == other.sc
    }
}

impl Eq for Pair {}

fn solve() {
    let stdin = io::stdin();
    let read = stdin.lock().lines();
    let mut iter = read.map(|x| x.unwrap()).peekable();

    let first_line = iter.next().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut arr: Vec<i64> = Vec::with_capacity(n);
    let second_line = iter.next().unwrap();
    arr.extend(second_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()));

    let mut x: Vec<Pair> = vec![Pair { fs: 0, sc: 0 }];
    let mut x2: Vec<i64> = vec![0; n + 1];
    
    for i in 1..=n {
        x.push(Pair { fs: x[i - 1].fs ^ arr[i - 1], sc: i });
    }

    for i in 0..=n {
        x2[i] = x[i].fs;
    }

    x.sort();

    // Process each query
    for _ in 0..q {
        let line = iter.next().unwrap();
        let mut parts = line.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search(&Pair { fs: c ^ x2[l - 1], sc: l }).unwrap_or_else(|x| x);
        if i1 == n + 1 || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search(&Pair { fs: x2[l - 1], sc: x[i1].sc + 1 }).unwrap_or_else(|x| x);
        if i2 == n + 1 || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t: usize = read_line().trim().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}

fn read_line() -> String {
    let stdin = io::stdin();
    let mut buffer = String::new();
    stdin.lock().read_line(&mut buffer).unwrap();
    buffer
}