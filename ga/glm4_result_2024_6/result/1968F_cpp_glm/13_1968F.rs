use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let nq: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];
        let arr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut x = vec![(0, 0); n + 1];
        let mut x2 = vec![0; n + 1];
        x[0] = (0, 0);
        for i in 1..=n {
            x[i] = (x[i - 1].0 ^ arr[i - 1], i);
            x2[i] = x[i].0;
        }
        x.sort_by(|a, b| a.0.cmp(&b.0));

        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let l = lr[0];
            let r = lr[1];
            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }
            let i1 = x.binary_search_by(|&(ref a, ref b)| {
                if a == &c ^ x2[l - 1] {
                    b.cmp(&l)
                } else {
                    a.cmp(&c ^ x2[l - 1])
                }
            }).unwrap_or_else(|i| i);

            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }
            let i2 = x.binary_search_by(|&(ref a, ref b)| {
                if a == &x2[l - 1] {
                    b.cmp(&(x[i1].1 + 1))
                } else {
                    a.cmp(&x2[l - 1])
                }
            }).unwrap_or_else(|i| i);

            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

fn main() {
    solve();
}