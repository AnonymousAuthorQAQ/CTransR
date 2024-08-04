use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Pair {
    fs: i64,
    sc: usize,
}

fn solve() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(Result::unwrap);
    let first_line = lines.next().unwrap();
    
    let mut iter = first_line.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = vec![0; n];
    let line = lines.next().unwrap();
    let arr_iter = line.split_whitespace();
    for (i, value) in arr_iter.enumerate() {
        arr[i] = value.parse::<i64>().unwrap();
    }

    let mut x: Vec<Pair> = vec![(0, 0).into(); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = Pair { fs: 0, sc: 0 };

    for i in 1..=n {
        x[i] = Pair { fs: x[i - 1].fs ^ arr[i - 1], sc: i };
    }
    
    for i in 0..=n {
        x2[i] = x[i].fs;
    }

    x.sort_by(|a, b| {
        match a.fs.cmp(&b.fs) {
            Ordering::Equal => a.sc.cmp(&b.sc),
            ord => ord,
        }
    });

    for _ in 0..q {
        let l_r_line = lines.next().unwrap();
        let mut lr_iter = l_r_line.split_whitespace();
        let l: usize = lr_iter.next().unwrap().parse().unwrap();
        let r: usize = lr_iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search(&Pair { fs: c ^ x2[l - 1], sc: l }).unwrap_err();
        if i1 == n + 1 || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search(&Pair { fs: x2[l - 1], sc: x[i1].sc + 1 }).unwrap_err();
        if i2 == n + 1 || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines().map(Result::unwrap);
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}

impl From<(i64, usize)> for Pair {
    fn from(tuple: (i64, usize)) -> Self {
        Pair { fs: tuple.0, sc: tuple.1 }
    }
}