use std::io::{self, Read};

#[derive(Debug)]
struct Query {
    l: usize,
    r: usize,
}

impl Query {
    fn new(l: usize, r: usize) -> Self {
        Self { l, r }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<&str> = input.trim().split('\n').collect();

    let t: usize = lines[0].parse().unwrap();
    let mut line_idx = 1;

    for _ in 0..t {
        let nq: Vec<usize> = lines[line_idx].split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];

        let mut arr = Vec::with_capacity(n);
        for num in lines[line_idx + 1].split_whitespace().map(|s| s.parse().unwrap()) {
            arr.push(num);
        }
        line_idx += 2;

        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            let lr: Vec<usize> = lines[line_idx].split_whitespace().map(|s| s.parse().unwrap()).collect();
            queries.push(Query::new(lr[0] - 1, lr[1] - 1));
            line_idx += 1;
        }

        let results = solve(n, q, &arr, &queries);
        for result in results {
            println!("{}", if result { "YES" } else { "NO" });
        }
        line_idx += 1;
    }
}

fn solve(n: usize, q: usize, arr: &[i32], queries: &[Query]) -> Vec<bool> {
    let mut x = vec![(0, 0)];
    let mut x2 = vec![0];

    for (i, &val) in arr.iter().enumerate() {
        x.push((x[i].0 ^ val, i + 1));
        x2.push(x[i + 1].0);
    }

    x.sort();

    let mut results = Vec::with_capacity(q);
    for &Query { l, r } in queries {
        let c = x2[r] ^ x2[l];
        if c == 0 {
            results.push(true);
            continue;
        }

        let i1 = match x.binary_search_by(|probe| (c ^ x2[l], probe.1).cmp(&(0, 0))) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n || x[i1].1 > r || x[i1].0 != (c ^ x2[l]) {
            results.push(false);
            continue;
        }

        let i2 = match x.binary_search_by(|probe| (x2[l], probe.1).cmp(&(0, x[i1].1))) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n || x[i2].1 > r || x[i2].0 != x2[l] {
            results.push(false);
        } else {
            results.push(true);
        }
    }

    results
}