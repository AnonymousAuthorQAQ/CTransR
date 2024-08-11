use std::collections::HashMap;
use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let nq: Vec<i32> = lines.next().unwrap().split_whitespace()
        .map(|x| x.parse().unwrap()).collect();
    let n = nq[0] as usize;
    let q = nq[1];

    let mut input_arr = lines.next().unwrap().split_whitespace();
    let arr: Vec<i32> = input_arr.map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(i32, usize)> = vec![(0, 0)];
    let mut x2: Vec<i32> = vec![0; n+1];
    let mut index = 1;
    for &num in &arr {
        x.push((x[index-1].0 ^ num, index));
        x2[index] = x[index].0;
        index += 1;
    }
    x.sort();
    
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    for _ in 0..q {
        let query: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let l = query[0] as usize;
        let r = query[1] as usize;

        let c = x2[r] ^ x2[l-1];
        if c == 0 {
            writeln!(handle, "YES").unwrap();
            continue;
        }

        let i1 = x.binary_search_by_key(&(c ^ x2[l-1]), |&(key, _)| key)
            .unwrap_or_else(|x| x);
        
        if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l-1]) {
            writeln!(handle, "NO").unwrap();
            continue;
        }

        let i2 = x.binary_search_by_key(&x2[l-1], |&(key, idx)| (key, x[i1].1+1))
            .unwrap_or_else(|x| x);

        if i2 == n+1 || x[i2].1 > r || x[i2].0 != x2[l-1] {
            writeln!(handle, "NO").unwrap();
        } else {
            writeln!(handle, "YES").unwrap();
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: i32 = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}