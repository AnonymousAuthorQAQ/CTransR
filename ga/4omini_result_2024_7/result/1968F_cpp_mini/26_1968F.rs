use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct Pair {
    value: i64,
    index: usize,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value == other.value {
            self.index.cmp(&other.index)
        } else {
            self.value.cmp(&other.value)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.index == other.index
    }
}

impl Eq for Pair {}

fn solve() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n_q: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = n_q[0];
        let q = n_q[1];
        
        let arr: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let mut x = vec![Pair { value: 0, index: 0 }; n + 1];
        let mut x2 = vec![0; n + 1];
        x[0] = Pair { value: 0, index: 0 };
        
        for i in 1..=n {
            x[i].value = x[i - 1].value ^ arr[i - 1];
            x[i].index = i;
        }
        
        for i in 0..=n {
            x2[i] = x[i].value;
        }
        
        x.sort();

        for _ in 0..q {
            let l_r: Vec<usize> = lines.next().unwrap().unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let l = l_r[0];
            let r = l_r[1];

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            // Binary search for the first element where `value == c ^ x2[l - 1]`
            let target = c ^ x2[l - 1];
            let i1 = x.binary_search(&Pair { value: target, index: l }).unwrap_err();
            
            if i1 == n + 1 || x[i1].index > r || x[i1].value != target {
                println!("NO");
                continue;
            }

            // Binary search for the first element where `value == x2[l - 1]`
            let i2 = x.binary_search(&Pair { value: x2[l - 1], index: x[i1].index + 1 }).unwrap_err();

            if i2 == n + 1 || x[i2].index > r || x[i2].value != x2[l - 1] {
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