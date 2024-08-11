use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve<R: BufRead>(lines: &mut R) {
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.trim().split_whitespace();
    
    let n: usize = nums.next().unwrap().parse().unwrap();
    let q: usize = nums.next().unwrap().parse().unwrap();
    
    let mut arr = Vec::with_capacity(n);
    let arr_line = lines.next().unwrap().unwrap();
    
    arr.extend(arr_line.trim().split_whitespace().map(|s| s.parse::<i64>().unwrap()));

    // Create prefix XOR and index pairs
    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i].0 = x[i - 1].0 ^ arr[i - 1];
        x[i].1 = i as i64;
    }
    
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    
    // Sort the x array based on the first element of the pairs
    x.sort();

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut query_nums = query_line.trim().split_whitespace();
        
        let l: usize = query_nums.next().unwrap().parse().unwrap();
        let r: usize = query_nums.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let target = c ^ x2[l - 1];

        // Binary search for i1
        let mut i1 = x.binary_search(&(target, 0)).unwrap_or_else(|e| e);
        
        if i1 == n + 1 || x[i1].1 > r as i64 || x[i1].0 != target {
            println!("NO");
            continue;
        }
        
        // Binary search for i2
        let mut i2 = x.binary_search(&(x2[l - 1], 0)).unwrap_or_else(|e| e);
        
        if i2 == n + 1 || x[i2].1 > r as i64 || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}