use std::io::{self, Read};
use std::cmp::{Ord, Ordering};

#[derive(Debug, Eq, PartialEq)]
struct Pair {
    fs: i64,
    sc: usize,
}

// Implement the Ord trait to allow sorting of pairs.
impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.fs == other.fs {
            self.sc.cmp(&other.sc)
        } else {
            self.fs.cmp(&other.fs)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();

    let first_line: Vec<usize> = lines.next().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();

    let n = first_line[0];
    let q = first_line[1];
    
    let mut arr: Vec<i64> = lines.next().unwrap().split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    
    let mut x: Vec<Pair> = vec![Pair { fs: 0, sc: 0 }];

    for i in 0..n {
        x.push(Pair {
            fs: x[i].fs ^ arr[i],
            sc: i + 1,
        });
    }

    let mut x2: Vec<i64> = x.iter().map(|pair| pair.fs).collect();
    
    // Sort x vector
    x.sort();

    for _ in 0..q {
        let query: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();

        let l = query[0];
        let r = query[1];
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        // Binary search for (c ^ x2[l - 1], l)
        let target1 = (c ^ x2[l - 1], l);
        let i1 = x.binary_search_by(|pair| {
            if pair.fs == target1.0 {
                pair.sc.cmp(&target1.1)
            } else {
                pair.fs.cmp(&target1.0)
            }
        }).unwrap_or_else(|x| x);

        if i1 == x.len() || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        // Binary search for (x2[l - 1], x[i1].sc + 1)
        let target2 = (x2[l - 1], x[i1].sc + 1);
        let i2 = x.binary_search_by(|pair| {
            if pair.fs == target2.0 {
                pair.sc.cmp(&target2.1)
            } else {
                pair.fs.cmp(&target2.0)
            }
        }).unwrap_or_else(|x| x);

        if i2 == x.len() || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        solve();
    }
}