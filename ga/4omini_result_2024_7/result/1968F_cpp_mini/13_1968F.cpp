use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
struct Pair {
    x: i64, // first element of the pair (XOR)
    index: usize, // index in the original array
}

// Implementing a function to compare pairs by their XOR value
impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.index.cmp(&other.index).then(Some(Ordering::Equal)),
            ord => Some(ord),
        }
    }
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.index == other.index
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut inputs = line.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = inputs.next().unwrap();
        let q = inputs.next().unwrap();

        let mut arr = Vec::new();
        let line = lines.next().unwrap().unwrap();
        for number in line.trim().split_whitespace() {
            arr.push(number.parse::<i64>().unwrap());
        }

        let mut x = vec![Pair { x: 0, index: 0 }; n + 1];
        let mut x2 = vec![0; n + 1];

        for i in 1..=n {
            x[i].x = x[i - 1].x ^ arr[i - 1];
            x[i].index = i;
        }

        for i in 0..=n {
            x2[i] = x[i].x;
        }

        x.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let mut lr = line.trim().split_whitespace().map(|s| s.parse::<usize>().unwrap());
            let l = lr.next().unwrap();
            let r = lr.next().unwrap();

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let target = c ^ x2[l - 1];
            let i1 = x.binary_search(&Pair { x: target, index: l }).unwrap_err();
            
            if i1 == n + 1 || x[i1].index > r || x[i1].x != target {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search(&Pair { x: x2[l - 1], index: x[i1].index + 1 }).unwrap_err();
            
            if i2 == n + 1 || x[i2].index > r || x[i2].x != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}