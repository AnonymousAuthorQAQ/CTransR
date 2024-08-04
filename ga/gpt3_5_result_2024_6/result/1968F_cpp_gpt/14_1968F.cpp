use std::io;
use std::cmp::Ordering;

type Pii = (i32, i32);

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace()
                            .map(|x| x.parse().unwrap())
                            .collect();

    let mut x: Vec<Pii> = vec![(0, 0); n+1];
    x[0] = (0, 0);
    for i in 1..=n {
        x[i] = (x[i-1].0 ^ arr[i-1], i as i32);
    }
    let mut x2: Vec<i32> = x.iter().map(|&(_, val)| val).collect();
    x.sort();

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: i32 = iter.next().unwrap().parse().unwrap();
        let r: i32 = iter.next().unwrap().parse().unwrap();

        let c = x2[r as usize] ^ x2[(l-1) as usize];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|&(val, _)| (c ^ x2[(l-1) as usize], val).cmp(&val)) {
            Ok(idx) => idx,
            Err(_) => n + 1,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[(l-1) as usize]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|&(val, idx)| (x2[(l-1) as usize], idx).cmp(&idx)) {
            Ok(idx) => idx,
            Err(_) => n + 1,
        };

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[(l-1) as usize] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}