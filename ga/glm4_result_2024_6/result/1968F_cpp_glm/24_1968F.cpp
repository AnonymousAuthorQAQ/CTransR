use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let (n, q) = lines.next().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
    let arr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();

    let mut x = vec![(0, 0)];
    let mut x2 = vec![0];
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
        x2.push(x[i].0);
    }

    x.sort();

    for _ in 0..q {
        let (l, r) = lines.next().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let mut heap = BinaryHeap::new();
        for i in l..=r {
            heap.push((!x2[i - 1] & x2[r] ^ x2[i - 1], i));
        }

        let mut found = false;
        while let Some((val, idx)) = heap.pop() {
            if val == c {
                found = true;
                break;
            }
        }

        if found {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

fn main() {
    let t = 1; // read number of test cases from input
    for _ in 0..t {
        solve();
    }
}