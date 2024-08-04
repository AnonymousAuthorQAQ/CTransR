use std::io::{self, BufRead};
use std::collections::binary_heap::BinaryHeap;

type ll = i64;
type pll = (ll, ll);
type pii = (i32, i32);

fn solve() {
    let (n, q) = read_two_ints();
    let arr: Vec<i32> = read_ints(n);
    let mut x = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for (i, &a) in arr.iter().enumerate() {
        x.push((x[i].0 ^ a, i as i32 + 1));
    }
    let x2 = x.iter().map(|&(ref fs, _)| fs).collect::<Vec<_>>();
    x.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

    for _ in 0..q {
        let (l, r) = read_two_ints();
        let c = x2[r as usize] ^ x2[(l - 1) as usize];
        if c == 0 {
            println!("YES");
            continue;
        }
        let mut heap = BinaryHeap::new();
        for i in l..=r {
            heap.push(x[i as usize].0);
            if heap.len() > 1 {
                let a = heap.pop().unwrap();
                if heap.peek() == Some(&a ^ c) {
                    println!("YES");
                    break;
                }
            }
        }
        if heap.len() <= 1 {
            println!("NO");
        }
    }
}

fn read_ints(n: i32) -> Vec<i32> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    line.split_whitespace().take(n as usize).map(|s| s.parse().unwrap()).collect()
}

fn read_two_ints() -> (i32, i32) {
    let line = io::stdin().lock().lines().next().unwrap().unwrap();
    let mut iter = line.split_whitespace();
    (iter.next().unwrap().parse().unwrap(), iter.next().unwrap().parse().unwrap())
}

fn main() {
    let t = read_two_ints().0;
    for _ in 0..t {
        solve();
    }
}