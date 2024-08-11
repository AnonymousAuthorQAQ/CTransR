use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Pair {
    first: i64,
    second: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Reading the number of test cases
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve<R: BufRead>(lines: &mut R) {
    let mut input = String::new();
    lines.read_line(&mut input).unwrap();
    let mut nums = input.split_whitespace();
    
    let n: usize = nums.next().unwrap().parse().unwrap();
    let q: usize = nums.next().unwrap().parse().unwrap();
    
    let mut arr = vec![0; n];
    input.clear();
    lines.read_line(&mut input).unwrap();
    for (i, num) in input.split_whitespace().enumerate() {
        arr[i] = num.parse::<i64>().unwrap();
    }
    
    let mut x = vec![Pair { first: 0, second: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];
    x[0] = Pair { first: 0, second: 0 };
    
    for i in 1..=n {
        x[i].first = x[i - 1].first ^ arr[i - 1];
        x[i].second = i;
    }
    
    for i in 0..=n {
        x2[i] = x[i].first;
    }
    
    x.sort_by(|a, b| a.first.cmp(&b.first));
    
    for _ in 0..q {
        input.clear();
        lines.read_line(&mut input).unwrap();
        let mut query = input.split_whitespace();
        let l: usize = query.next().unwrap().parse().unwrap();
        let r: usize = query.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search(&Pair { first: c ^ x2[l - 1], second: l })
            .unwrap_or_else(|e| e);
        
        if i1 == n + 1 || x[i1].second > r || x[i1].first != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search(&Pair { first: x2[l - 1], second: x[i1].second + 1 })
            .unwrap_or_else(|e| e);
        
        if i2 == n + 1 || x[i2].second > r || x[i2].first != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

// Implementing PartialEq and Ord traits for Pair to use binary_search
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.first != other.first {
            self.first.cmp(&other.first)
        } else {
            self.second.cmp(&other.second)
        }
    }
}

impl Eq for Pair {}