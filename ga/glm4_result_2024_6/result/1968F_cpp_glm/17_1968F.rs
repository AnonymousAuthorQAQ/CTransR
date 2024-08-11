use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let q: usize = tokens.next().unwrap().parse().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        let mut x = vec![(0, 0)];
        let mut x2 = vec![0];
        for (i, &num) in arr.iter().enumerate() {
            x.push((x[i].0 ^ num, i + 1));
            x2.push(x[i + 1].0);
        }
        x2.insert(0, 0);
        
        x.sort_unstable();
        
        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut tokens = input.trim().split_whitespace();
            let l: usize = tokens.next().unwrap().parse().unwrap();
            let r: usize = tokens.next().unwrap().parse().unwrap();
            
            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let i1 = match x.binary_search_by(|&(val, idx)| {
                if val == (c ^ x2[l - 1]) {
                    Ordering::Equal
                } else {
                    val.cmp(&(c ^ x2[l - 1]))
                }
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }
            
            let i2 = match x.binary_search_by(|&(val, idx)| {
                if val == x2[l - 1] {
                    Ordering::Equal
                } else {
                    val.cmp(&x2[l - 1])
                }
            }) {
                Ok(i) => i,
                Err(i) => i,
            };
            
            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}