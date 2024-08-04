use std::io::{self, Read};
use std::collections::BinaryHeap;

fn solve(n: usize, q: usize, arr: Vec<i32>, queries: Vec<(usize, usize)>) {
    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));

    for (i, &num) in arr.iter().enumerate() {
        x.push((x[i].0 ^ num, i + 1));
    }

    let mut x2 = Vec::from_iter(x.iter().map(|&(val, _)| val));

    let mut x = Vec::from_iter(x);
    x.sort();

    for (l, r) in queries {
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|&(val, idx)| (val ^ x2[l - 1], idx)) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|&(val, idx)| (val, idx)) {
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let t = lines.next().unwrap().parse::<usize>().unwrap();

    for _ in 0..t {
        let mut iter = lines.next().unwrap().split_whitespace();
        let n = iter.next().unwrap().parse::<usize>().unwrap();
        let q = iter.next().unwrap().parse::<usize>().unwrap();
        let arr = iter.map(|s| s.parse::<i32>().unwrap()).collect();

        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
            let mut iter = lines.next().unwrap().split_whitespace();
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            queries.push((l, r));
        }

        solve(n, q, arr, queries);
    }
}