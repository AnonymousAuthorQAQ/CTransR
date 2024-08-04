use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, q) = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
    let arr = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>();
    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for (i, &num) in arr.iter().enumerate() {
        x.push((x[i].0 ^ num, i + 1));
    }
    let x2: Vec<usize> = x.iter().map(|&(a, _)| a).collect();

    for _ in 0..q {
        let (l, r) = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        let mut heap = BinaryHeap::new();
        for &(_, idx) in &x {
            if idx >= l && idx <= r {
                heap.push((-(idx as isize), idx));
            }
        }
        let mut found = false;
        while let Some((_, idx)) = heap.pop() {
            if x[idx - 1].0 == c ^ x2[l - 1] {
                let mut heap2 = BinaryHeap::new();
                for &(_, idx2) in &x {
                    if idx2 >= idx && idx2 <= r {
                        heap2.push((-(idx2 as isize), idx2));
                    }
                }
                if let Some((_, idx2)) = heap2.pop() {
                    if x[idx2 - 1].0 == x2[l - 1] {
                        found = true;
                        break;
                    }
                }
            }
        }
        println!("{}", if found { "YES" } else { "NO" });
    }
}

fn main() {
    let t = io::stdin().lock().lines().next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}