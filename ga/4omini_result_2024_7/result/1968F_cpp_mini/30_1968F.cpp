use std::io::{self, BufRead};
use std::cmp::{self, Ordering};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let q = nums.next().unwrap();

        let mut arr = Vec::new();
        let arr_line = lines.next().unwrap().unwrap();
        arr.extend(arr_line.split_whitespace().map(|s| s.parse::<i64>().unwrap()));

        let mut x = vec![(0i64, 0); n + 1];
        let mut x2 = vec![0; n + 1];
        x[0] = (0, 0);

        for i in 1..=n {
            x[i] = (x[i - 1].0 ^ arr[i - 1], i);
        }

        for i in 0..=n {
            x2[i] = x[i].0;
        }

        x.sort_by(|a, b| a.cmp(b));

        for _ in 0..q {
            let query_line = lines.next().unwrap().unwrap();
            let mut bounds = query_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
            let l = bounds.next().unwrap();
            let r = bounds.next().unwrap();

            let c = x2[r] ^ x2[l - 1];

            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |pair| pair.0).unwrap_or_else(|e| e);
            
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by_key(&x2[l - 1], |pair| pair.0).unwrap_or_else(|e| e);
            
            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}