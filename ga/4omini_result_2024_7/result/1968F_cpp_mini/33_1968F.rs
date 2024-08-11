use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = Vec::new();
    input.clear();
    reader.read_line(&mut input).unwrap();
    arr.extend(input.split_whitespace().map(|x| x.parse::<i64>().unwrap()));

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);

    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort_unstable();

    for _ in 0..q {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut query_iter = input.split_whitespace();
        
        let l: usize = query_iter.next().unwrap().parse().unwrap();
        let r: usize = query_iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let mut lower_bound = |val: i64, start: usize| {
            let mut low = start;
            let mut high = n + 1;
            while low < high {
                let mid = (low + high) / 2;
                match x[mid].0.cmp(&val) {
                    Ordering::Less => low = mid + 1,
                    _ => high = mid,
                }
            }
            low
        };

        let i1 = lower_bound(c ^ x2[l - 1], 0);
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = lower_bound(x2[l - 1], i1 + 1);
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}