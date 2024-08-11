use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    
    let t: usize = read_line(&input).trim().parse().unwrap();
    for _ in 0..t {
        let line = read_line(&input);
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        
        let arr: Vec<u32> = read_line(&input)
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let mut x: Vec<(u32, usize)> = vec![(0, 0)];
        for i in 1..=n {
            x.push((x[i - 1].0 ^ arr[i - 1], i));
        }
        
        let mut x2: Vec<u32> = x.iter().map(|&(v, _)| v).collect();
        x.sort();
        
        for _ in 0..q {
            let line = read_line(&input);
            let mut iter = line.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            
            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(v, _)| v).unwrap_or_else(|x| x);
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }
            
            let i2 = x.binary_search_by_key(&x2[l - 1], |&(v, _)| v).unwrap_or_else(|x| x);
            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

fn read_line<R: BufRead>(input: &R) -> String {
    let mut line = String::new();
    input.read_line(&mut line).unwrap();
    line.trim().to_string()
}