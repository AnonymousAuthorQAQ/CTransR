use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nq: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = nq[0];
    let q = nq[1];

    let arr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

    let mut x: Vec<(usize, usize)> = vec![(0, 0)];
    let mut x2: Vec<usize> = vec![0];

    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
        x2.push(x[i].0);
    }

    x.sort();

    for _ in 0..q {
        let lr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let l = lr[0];
        let r = lr[1];

        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by_key(&c ^ x2[l - 1], |&(ref a, _)| *a).unwrap_or_else(|x| x);

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by_key(&x2[l - 1], |&(ref a, ref b)| (*a, *b)).unwrap_or_else(|x| x);

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let t = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}