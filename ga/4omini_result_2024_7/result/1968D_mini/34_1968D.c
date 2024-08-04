use std::cmp::{min, max};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (n, k, mut a, mut b) = (params[0], params[1], params[2], params[3]);
        let m = min(n, k);
        
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        for x in 1..=n {
            arr[x] = lines.next().unwrap().unwrap().parse().unwrap();
        }
        
        for x in 1..=n {
            aa[x] = lines.next().unwrap().unwrap().parse().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = max(p1, aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = max(p2, aa[b] * (k - x + 1) + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}