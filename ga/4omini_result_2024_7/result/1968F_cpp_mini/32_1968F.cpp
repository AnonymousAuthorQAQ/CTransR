use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Pair {
    fs: i64,
    sc: usize,
}

fn solve() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    reader.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    input.clear();
    reader.read_line(&mut input).unwrap();
    for x in input.split_whitespace() {
        arr.push(x.parse::<i64>().unwrap());
    }

    let mut x = vec![Pair { fs: 0, sc: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];
    for i in 1..=n {
        x[i].fs = x[i - 1].fs ^ arr[i - 1];
        x[i].sc = i;
    }
    for i in 0..=n {
        x2[i] = x[i].fs;
    }

    x.sort_by(|a, b| {
        if a.fs == b.fs {
            a.sc.cmp(&b.sc)
        } else {
            a.fs.cmp(&b.fs)
        }
    });

    for _ in 0..q {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let target = c ^ x2[l - 1];

        let i1 = x.binary_search_by(|p| {
            if p.fs == target {
                Ordering::Equal
            } else if p.fs < target {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        match i1 {
            Ok(idx) => {
                if x[idx].sc > r {
                    println!("NO");
                    continue;
                }
            },
            Err(idx) => {
                let idx = idx - 1;
                if idx < n && x[idx].sc <= r && x[idx].fs == target {
                    // Found an appropriate i1
                } else {
                    println!("NO");
                    continue;
                }
            }
        }

        let i2 = x.binary_search_by(|p| {
            if p.fs == x2[l - 1] {
                Ordering::Equal
            } else if p.fs < x2[l - 1] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        match i2 {
            Ok(idx) => {
                if x[idx].sc > r {
                    println!("NO");
                } else {
                    println!("YES");
                }
            },
            Err(idx) => {
                let idx = idx - 1;
                if idx < n && x[idx].sc <= r && x[idx].fs == x2[l - 1] {
                    println!("YES");
                } else {
                    println!("NO");
                }
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = String::new();

    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}