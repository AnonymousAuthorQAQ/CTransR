use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, q): (usize, usize) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect_tuple().unwrap();
    let arr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut x: Vec<(usize, usize)> = vec![(0, 0)];
    let mut x2: Vec<usize> = vec![0];

    for (i, &a) in arr.iter().enumerate() {
        x.push((x[i].0 ^ a, i + 1));
        x2.push(x[i + 1].0);
    }

    x2.sort();

    for _ in 0..q {
        let (l, r): (usize, usize) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect_tuple().unwrap();

        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|&(ref x, ref y)| (x ^ x2[l - 1], y)) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|&(ref x, ref y)| (x ^ x2[l - 1], y)) {
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
    let t: usize = io::stdin().lock().lines().next().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}