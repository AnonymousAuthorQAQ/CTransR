use std::io;
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Query {
    fs: i64,
    sc: i32,
}

impl Query {
    fn new(fs: i64, sc: i32) -> Query {
        Query { fs, sc }
    }
}

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let nq: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = nq[0];
    let q = nq[1];

    let arr: Vec<i64> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();

    let mut x: Vec<i64> = vec![0];
    for i in 0..n {
        x.push(x[i] ^ arr[i]);
    }
    
    let mut x2: Vec<i64> = x.clone();
    let mut x: Vec<Query> = x.into_iter().enumerate().map(|(i, fs)| Query::new(fs, i as i32)).collect();

    x.sort();

    while q > 0 {
        let lrb: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let l = lrb[0];
        let r = lrb[1];

        let c = x2[r as usize] ^ x2[l as usize - 1];

        if c == 0 {
            println!("YES");
            q -= 1;
            continue;
        }

        let i1 = {
            let mut heap = BinaryHeap::new();
            for i in 0..=r as usize {
                heap.push(Query::new(x2[i] ^ x2[l as usize - 1], i as i32));
            }
            loop {
                if heap.is_empty() {
                    break None;
                }
                let top = heap.pop().unwrap();
                if top.sc <= r {
                    break Some(top.sc);
                }
            }
        };

        let i2 = {
            let mut heap = BinaryHeap::new();
            for i in 0..=r as usize {
                heap.push(Query::new(x2[l as usize - 1], i as i32));
            }
            loop {
                if heap.is_empty() {
                    break None;
                }
                let top = heap.pop().unwrap();
                if top.sc <= r && x2[top.sc as usize] == x2[l as usize - 1] {
                    break Some(top.sc);
                }
            }
        };

        if i1.is_none() || i2.is_none() || x[i1.unwrap() as usize].fs != (c ^ x2[l as usize - 1]) || x[i2.unwrap() as usize].fs != x2[l as usize - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
        q -= 1;
    }
}

fn main() {
    let t: i32 = read!("{}\n");
    for _ in 0..t {
        solve();
    }
}