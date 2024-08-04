use std::io;
use std::io::prelude::*;
use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut iter = lines.map(|l| l.unwrap());

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let nq: Vec<usize> = iter.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];

        let arr: Vec<usize> = iter.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let mut x: Vec<(usize, usize)> = vec![(0, 0)];
        let mut x2: Vec<usize> = vec![0];

        for i in 1..=n {
            x.push((x[i - 1].0 ^ arr[i - 1], i));
            x2.push(x[i].0);
        }

        x.sort_by(|a, b| a.0.cmp(&b.0));

        for _ in 0..q {
            let lr: Vec<usize> = iter.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
            let l = lr[0] + 1;
            let r = lr[1] + 1;

            let c = x2[r] ^ x2[l - 1];

            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = {
                let target = c ^ x2[l - 1];
                let mut left = 0;
                let mut right = n;

                while left < right {
                    let mid = (left + right) / 2;
                    if x[mid].0 < target {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }

                left
            };

            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = {
                let target = x2[l - 1];
                let mut left = 0;
                let mut right = n;

                while left < right {
                    let mid = (left + right) / 2;
                    if x[mid].0 < target {
                        left = mid + 1;
                    } else {
                        right = mid;
                    }
                }

                left
            };

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