use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut split_input = input.trim().split_whitespace();
    let n: usize = split_input.next().unwrap().parse().unwrap();
    let q: usize = split_input.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<u32> = input.trim().split_whitespace()
                                .map(|x| x.parse().unwrap())
                                .collect();

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = (0, 0);
    for i in 1..=n {
        x[i] = (x[i-1].0 ^ arr[i-1], i);
    }
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    x.sort();

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut split_query = input.trim().split_whitespace();
        let l: usize = split_query.next().unwrap().parse().unwrap();
        let r: usize = split_query.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        let i1 = match x.binary_search_by_key(&(c ^ x2[l - 1]), |&(val, _)| val) {
            Ok(idx) => idx,
            Err(_) => n + 1,
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by_key(&x2[l - 1], |&(val, idx)| idx) {
            Ok(idx) => idx,
            Err(_) => n + 1,
        };

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}