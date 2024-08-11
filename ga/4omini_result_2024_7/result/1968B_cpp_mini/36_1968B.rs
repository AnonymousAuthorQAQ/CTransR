use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let xk: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let x = xk[0];
        let k = xk[1];
        
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        for ch in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] as char == ch {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}