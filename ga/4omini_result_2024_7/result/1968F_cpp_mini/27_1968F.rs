use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Pair {
    xor_value: i64,
    index: usize,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.xor_value == other.xor_value
    }
}

impl Eq for Pair {}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.xor_value.cmp(&other.xor_value).then_with(|| self.index.cmp(&other.index))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = input.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let (n, q): (usize, usize) = {
            let line = input.next().unwrap();
            let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
            (iter.next().unwrap(), iter.next().unwrap())
        };
        
        let arr: Vec<i64> = input.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        
        let mut prefix_xors = vec![Pair { xor_value: 0, index: 0 }];
        
        for i in 1..=n {
            let new_xor = prefix_xors[i - 1].xor_value ^ arr[i - 1];
            prefix_xors.push(Pair { xor_value: new_xor, index: i });
        }
        
        let mut prefix_values = prefix_xors.iter().map(|p| p.xor_value).collect::<Vec<i64>>();
        
        // Sort the pairs based on XOR values and indices
        prefix_xors.sort();
        
        for _ in 0..q {
            let (l, r): (usize, usize) = {
                let line = input.next().unwrap();
                let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
                (iter.next().unwrap(), iter.next().unwrap())
            };

            let c = prefix_values[r] ^ prefix_values[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let target_xor = c ^ prefix_values[l - 1];
            let i1 = prefix_xors.binary_search(&Pair { xor_value: target_xor, index: l }).unwrap_err();
            
            if i1 == prefix_xors.len() || prefix_xors[i1].index > r || prefix_xors[i1].xor_value != target_xor {
                println!("NO");
                continue;
            }
            
            let i2 = prefix_xors.binary_search(&Pair { xor_value: prefix_values[l - 1], index: prefix_xors[i1].index + 1 }).unwrap_err();
            
            if i2 == prefix_xors.len() || prefix_xors[i2].index > r || prefix_xors[i2].xor_value != prefix_values[l - 1] {
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