use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: i32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let nq: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = nq[0] as usize;
        let q = nq[1] as usize;
        
        let arr: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        
        let mut x: Vec<(i32, usize)> = vec![(0, 0); n+1];
        for i in 1..=n {
            x[i] = (x[i-1].0 ^ arr[i-1], i);
        }
        
        let mut x2: Vec<i32> = vec![0; n+1];
        for i in 0..=n {
            x2[i] = x[i].0;
        }
        
        x.sort();
        
        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            let l = lr[0];
            let r = lr[1];
            
            let c = x2[r] ^ x2[l-1];
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let i1 = x.binary_search_by_key(&(c ^ x2[l-1]), |&(val, _)| val).unwrap_or_else(|x| x);
            if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l-1]) {
                println!("NO");
                continue;
            }
            
            let i2 = x.binary_search_by_key(&x2[l-1], |&(val, idx)| idx).unwrap_or_else(|x| x);
            if i2 == n+1 || x[i2].1 > r || x[i2].0 != x2[l-1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}