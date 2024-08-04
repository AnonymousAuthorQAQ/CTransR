use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Pair {
    fs: i64,
    sc: usize,
}

fn solve() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut input = String::new();
    
    reader.read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();
    
    let mut arr = vec![0; n];
    input.clear();
    reader.read_line(&mut input).unwrap();
    for (i, part) in input.split_whitespace().enumerate() {
        arr[i] = part.parse().unwrap();
    }
    
    let mut x = vec![Pair { fs: 0, sc: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = Pair {
            fs: x[i - 1].fs ^ arr[i - 1],
            sc: i,
        };
        x2[i] = x[i].fs;
    }
    
    x[0] = Pair { fs: 0, sc: 0 }; // initializing the first element
    x.sort_by(|a, b| a.fs.cmp(&b.fs));
    
    for _ in 0..q {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let l: usize = parts.next().unwrap().parse().unwrap();
        let r: usize = parts.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search(&Pair { fs: c ^ x2[l - 1], sc: l })
            .unwrap_or_else(|x| x); // using either the found index or the insertion point

        if i1 == n + 1 || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search(&Pair { fs: x2[l - 1], sc: x[i1].sc + 1 })
            .unwrap_or_else(|x| x); // using either the found index or the insertion point

        if i2 == n + 1 || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}