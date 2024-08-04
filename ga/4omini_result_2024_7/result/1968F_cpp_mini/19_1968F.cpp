use std::io::{self, BufRead};
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock();
    let mut buffer = String::new();
    
    // Read number of test cases
    reader.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    buffer.clear();
    
    for _ in 0..t {
        // Read n and q
        reader.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        buffer.clear();
        
        // Read array elements
        let mut arr: Vec<i64> = Vec::with_capacity(n);
        reader.read_line(&mut buffer).unwrap();
        arr.extend(buffer.split_whitespace().map(|x| x.parse::<i64>().unwrap()));
        buffer.clear();
        
        let mut x = vec![(0, 0); n + 1];
        let mut x2 = vec![0; n + 1];
        
        for i in 1..=n {
            x[i] = (x[i - 1].0 ^ arr[i - 1], i as i64);
        }
        
        for i in 0..=n {
            x2[i] = x[i].0;
        }
        
        x.sort_by_key(|k| k.0);
        
        for _ in 0..q {
            // Read l and r
            reader.read_line(&mut buffer).unwrap();
            let mut iter = buffer.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            buffer.clear();
            
            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let target = c ^ x2[l - 1];
            let i1 = x.binary_search_by_key(&(target, l as i64), |&(v, _)| (v, 0)).unwrap_err();
            if i1 == x.len() || x[i1].1 > r as i64 || x[i1].0 != target {
                println!("NO");
                continue;
            }
            
            let i2 = x.binary_search_by_key(&(x2[l - 1], x[i1].1 + 1), |&(v, _)| (v, 0)).unwrap_err();
            if i2 == x.len() || x[i2].1 > r as i64 || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}