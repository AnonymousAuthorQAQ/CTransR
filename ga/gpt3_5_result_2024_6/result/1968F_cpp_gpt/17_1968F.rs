use std::collections::HashMap;
use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = values[0];
    let q = values[1];

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut x: Vec<(i32, usize)> = Vec::with_capacity(n + 1);
    let mut x2: Vec<i32> = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    for i in 0..=n {
        x2.push(x[i].0);
    }
    x.sort();

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let lr: Vec<i32> = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let l = lr[0] as usize;
        let r = lr[1] as usize;

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        let mut map: HashMap<i32, usize> = HashMap::new();
        for i in 0..=n {
            map.insert(x[i].0, x[i].1);
        }
        let i1 = map.get(&(c ^ x2[l - 1])).unwrap_or(&0);

        if *i1 == 0 || *i1 > r || x[*i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        let i2 = map.get(&x2[l - 1]).unwrap_or(&0);

        if *i2 == 0 || *i2 > r || x[*i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}