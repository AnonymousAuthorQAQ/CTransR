use std::io::{self, Write};
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        
        let mut arr = Vec::with_capacity(n);
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        for num in input.split_whitespace() {
            arr.push(num.parse::<i64>().unwrap());
        }

        let mut x = Vec::with_capacity(n + 1);
        let mut x2 = vec![0; n + 1];
        x.push((0, 0)); // Initialize x[0]

        for i in 1..=n {
            x.push((x[i - 1].0 ^ arr[i - 1], i as i64));
        }

        for i in 0..=n {
            x2[i] = x[i].0;
        }

        // Sort x based on the first element of the tuple
        x.sort_by(|a, b| {
            match a.0.cmp(&b.0) {
                Ordering::Equal => a.1.cmp(&b.1),
                other => other,
            }
        });

        for _ in 0..q {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            
            let c = x2[r] ^ x2[l - 1];
            
            if c == 0 {
                println!("YES");
                continue;
            }

            if let Ok(i1) = x.binary_search(&(c ^ x2[l - 1], l as i64)) {
                let i1 = i1.unwrap();
                if x[i1].1 > r as i64 {
                    println!("NO");
                    continue;
                }

                if x[i1].0 != (c ^ x2[l - 1]) {
                    println!("NO");
                    continue;
                }

                let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_err();
                
                if i2 < x.len() && x[i2].1 <= r as i64 && x[i2].0 == x2[l - 1] {
                    println!("YES");
                } else {
                    println!("NO");
                }
            } else {
                println!("NO");
            }
        }
    }
}