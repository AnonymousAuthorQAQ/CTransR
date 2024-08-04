use std::io;
use std::cmp::{max, min};
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let params: Vec<usize> = input.split_whitespace()
                                       .map(|x| x.parse().unwrap())
                                       .collect();
        let n = params[0];
        let k = params[1];
        let mut a = params[2] - 1; // Converts to zero-indexed
        let mut b = params[3] - 1;

        let m = min(n, k);
        let mut arr = vec![0; n];
        let mut aa = vec![0; n];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            aa[i] = val.parse().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            arr[i] = val.parse().unwrap();
        }

        let mut p1 = 0_i64;
        let mut p2 = 0_i64;
        let mut p1_ = 0_i64;
        let mut p2_ = 0_i64;

        for x in 0..m {
            p1 = max(p1, aa[a] * (k - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a] - 1; // Converts to zero-indexed
            
            p2 = max(p2, aa[b] * (k - x as i64 + 1) + p2_);
            p2_ += aa[b];
            b = arr[b] - 1; // Converts to zero-indexed
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