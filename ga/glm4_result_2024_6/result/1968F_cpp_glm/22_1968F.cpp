use std::io::{self, BufRead};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Query {
    l: usize,
    r: usize,
    i: usize,
}

impl Query {
    fn new(l: usize, r: usize, i: usize) -> Self {
        Query { l, r, i }
    }
}

impl Ord for Query {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.r.cmp(&other.r)
    }
}

impl PartialOrd for Query {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Query")
            .field("l", &self.l)
            .field("r", &self.r)
            .field("i", &self.i)
            .finish()
    }
}

fn solve() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut lines = lines.map(|line| line.unwrap());

    let (n, q) = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect_tuple().unwrap();
    let arr = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>();

    let mut x = vec![0; n + 1];
    for i in 1..=n {
        x[i] = x[i - 1] ^ arr[i - 1];
    }
    let x2 = x.clone();

    let mut heap = BinaryHeap::new();
    for (i, &r) in x.iter().enumerate() {
        heap.push(Query::new(i, i, i));
    }

    while let Some(Query { l, r, i }) = heap.pop() {
        if l > r {
            continue;
        }
        if x[l] == x2[r] {
            println!("YES");
        } else {
            let l1 = x2[l].checked_shr(1).map(|v| *heap.get_binary_heap().binary_search_by(|&Query { l: l1, r: r1, i: _ }| l1.cmp(&v)).unwrap());
            let r1 = x2[r].checked_shr(1).map(|v| *heap.get_binary_heap().binary_search_by(|&Query { l: l1, r: r1, i: _ }| r1.cmp(&v)).unwrap());
            if l1.is_some() && r1.is_some() {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut lines = lines.map(|line| line.unwrap());

    let t = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}