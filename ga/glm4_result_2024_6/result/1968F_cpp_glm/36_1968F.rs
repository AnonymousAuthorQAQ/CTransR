use std::io::{self, Read};
use std::collections::binary_heap::BinaryHeap;
use std::cmp::Ordering;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.trim().split_whitespace();
    let n: usize = input.next().unwrap().parse().unwrap();
    let q: usize = input.next().unwrap().parse().unwrap();

    let mut arr = Vec::with_capacity(n);
    for _ in 0..n {
        arr.push(input.next().unwrap().parse().unwrap());
    }

    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    let mut x2 = Vec::with_capacity(n + 1);
    for i in 0..=n {
        x2.push(x[i].0);
    }
    x2.sort();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut input = input.trim().split_whitespace();
        let l: usize = input.next().unwrap().parse().unwrap();
        let r: usize = input.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|&(ref a, ref b)| {
            if a == &(c ^ x2[l - 1]) {
                Ordering::Equal
            } else {
                Ordering::from(a.cmp(&(c ^ x2[l - 1])))
            }
        }) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|&(ref a, ref b)| {
            if a == &x2[l - 1] {
                Ordering::Equal
            } else {
                Ordering::from(a.cmp(&x2[l - 1]))
            }
        }) {
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}