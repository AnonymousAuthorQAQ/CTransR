use std::io::{self, Read};
use std::collections::BinaryHeap;

struct Pair {
    fs: i64,
    sc: usize,
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve(&mut iter);
    }
}

fn solve(iter: &mut std::str::SplitWhitespace) {
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut arr = vec![0; n];

    for i in 0..n {
        arr[i] = iter.next().unwrap().parse().unwrap();
    }

    let mut x = vec![Pair { fs: 0, sc: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = Pair {
            fs: x[i - 1].fs ^ arr[i - 1],
            sc: i,
        };
    }

    for i in 0..=n {
        x2[i] = x[i].fs;
    }

    x.sort_by_key(|p| p.fs);
    
    for _ in 0..q {
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        let c = x2[r] ^ x2[l - 1];

        if c == 0 {
            println!("YES");
            continue;
        }

        let mut i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |p| p.fs).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let mut i2 = x.binary_search_by_key(&x2[l - 1], |p| p.fs).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}