use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace()
                              .map(|x| x.parse().unwrap())
                              .collect();

    let mut x: Vec<(i32, usize)> = vec![(0, 0); n+1];
    x[0] = (0, 0);
    for i in 1..=n {
        x[i] = (x[i-1].0 ^ arr[i-1], i);
    }
    let mut x2: Vec<i32> = x.iter().map(|v| v.0).collect();
    x.sort();

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l-1];
        if c == 0 {
            println!("YES");
            continue;
        }
        let i1 = x.binary_search_by(|v| (c ^ x2[l-1], l).cmp(&(v.0, v.1)))
                  .unwrap_or_else(|x| x);
        
        if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l-1]) {
            println!("NO");
            continue;
        }
        let i2 = x.binary_search_by(|v| (x2[l-1], x[i1].1 + 1).cmp(&(v.0, v.1)))
                  .unwrap_or_else(|x| x);
        
        if i2 == n+1 || x[i2].1 > r || x[i2].0 != x2[l-1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}