use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
struct Pair {
    fs: i64,
    sc: usize,
}

// Custom implementation of PartialOrd and Ord for Pair to allow sorting
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.fs.cmp(&other.fs) {
            Ordering::Equal => self.sc.cmp(&other.sc).then(Ordering::Equal),
            other => Some(other),
        }
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let t: usize = first_line.trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut arr: Vec<i64> = Vec::new();
        let arr_line = lines.next().unwrap().unwrap();
        arr.extend(arr_line.split_whitespace().map(|x| x.parse().unwrap()));

        let mut x: Vec<Pair> = vec![Pair { fs: 0, sc: 0 }; n + 1];
        let mut x2: Vec<i64> = vec![0; n + 1];

        for i in 1..=n {
            x[i] = Pair { fs: x[i - 1].fs ^ arr[i - 1], sc: i };
        }
        
        for i in 0..=n {
            x2[i] = x[i].fs;
        }

        x.sort();

        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let mut i1 = x.binary_search(&Pair { fs: c ^ x2[l - 1], sc: l }).unwrap_or_else(|e| e);
            if i1 == n + 1 || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let mut i2 = x.binary_search(&Pair { fs: x2[l - 1], sc: x[i1].sc + 1 }).unwrap_or_else(|e| e);
            if i2 == n + 1 || x[i2].sc > r || x[i2].fs != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

fn main() {
    solve();
}