use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Query {
    l: usize,
    r: usize,
    i: usize,
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r.cmp(&other.r)
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve() {
    let n = read_int();
    let q = read_int();
    let mut arr = read_ints(n);
    let mut x = vec![0; n + 1];
    let mut x2 = vec![0; n + 1];
    let mut queries = Vec::with_capacity(q);

    for i in 1..=n {
        x[i] = x[i - 1] ^ arr[i - 1];
        x2[i] = x[i];
    }

    for _ in 0..q {
        let l = read_int() - 1;
        let r = read_int();
        let i = queries.len();
        queries.push(Query { l, r, i });
    }

    queries.sort_unstable();

    let mut j = 0;
    for query in queries {
        while j < query.r {
            x2[j + 1] = x2[j] ^ arr[j];
            j += 1;
        }

        let c = x2[query.r] ^ x2[query.l];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by_key(&c ^ x2[query.l], |&(val, _)| val) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n || x[i1].1 > query.r || x[i1].0 != (c ^ x2[query.l]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by_key(&x2[query.l], |&(val, _)| val) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n || x[i2].1 > query.r || x[i2].0 != x2[query.l] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let mut reader = io::BufReader::new(stdin.lock());

    let t = read_int();
    for _ in 0..t {
        solve();
    }
}

fn read_ints(n: usize) -> Vec<i32> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn read_int() -> i32 {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse().unwrap()
}