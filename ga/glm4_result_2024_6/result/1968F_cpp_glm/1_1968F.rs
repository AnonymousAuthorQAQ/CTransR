use std::io::{self, BufRead};
use std::collections::binary_heap::BinaryHeap;

fn solve(n: usize, q: usize, arr: Vec<usize>) {
    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for (i, &val) in arr.iter().enumerate() {
        x.push((x[i].0 ^ val, i + 1));
    }
    let mut x2 = x.iter().map(|&(fs, _)| fs).collect::<Vec<_>>();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut heap = BinaryHeap::new();
        for &val in &x2[l - 1..r] {
            heap.push(val);
        }

        let mut found = false;
        for &val in &x2[l - 1..r] {
            if let Some(&top) = heap.peek() {
                if top ^ val == c {
                    found = true;
                    break;
                }
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
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let nq = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
        let (n, q) = (nq.next().unwrap(), nq.next().unwrap());
        let arr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

        solve(n, q, arr);
    }
}