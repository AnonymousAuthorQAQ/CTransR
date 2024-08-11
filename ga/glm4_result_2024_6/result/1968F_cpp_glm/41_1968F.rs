use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, q) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
    let arr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];

    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort();

    for _ in 0..q {
        let (l, r) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by_key(&(c ^ x2[l - 1]), |&(ref a, _)| a) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by_key(&(x2[l - 1], x[i1].1 + 1), |&(ref a, ref b)| (a, b)) {
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
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}