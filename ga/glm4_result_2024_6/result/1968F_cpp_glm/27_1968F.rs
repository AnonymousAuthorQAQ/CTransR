use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n = iter.next().unwrap().parse::<i32>().unwrap();
        let q = iter.next().unwrap().parse::<i32>().unwrap();

        let mut arr = Vec::with_capacity(n as usize);
        for _ in 0..n {
            arr.push(lines.next().unwrap().unwrap().parse::<i32>().unwrap());
        }

        let mut x = Vec::with_capacity(n as usize + 1);
        x.push((0, 0));
        for i in 1..=n {
            x.push((x[i - 1].0 ^ arr[i as usize - 1], i));
        }

        let mut x2 = Vec::with_capacity(n as usize + 1);
        for val in x.iter() {
            x2.push(val.0);
        }

        x.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        for _ in 0..q {
            let mut line = lines.next().unwrap().unwrap();
            let mut iter = line.split_whitespace();
            let l = iter.next().unwrap().parse::<i32>().unwrap();
            let r = iter.next().unwrap().parse::<i32>().unwrap();

            let c = x2[r as usize] ^ x2[(l - 1) as usize];
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by(|probe| {
                (probe.0 ^ x2[(l - 1) as usize]).cmp(&probe.0)
            }).unwrap_err();

            if i1 == n as usize || x[i1].1 > r || x[i1].0 != (c ^ x2[(l - 1) as usize]) {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by(|probe| {
                (probe.0 ^ x2[(l - 1) as usize]).cmp(&probe.0)
            }).unwrap_err();

            if i2 == n as usize || x[i2].1 > r || x[i2].0 != x2[(l - 1) as usize] {
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