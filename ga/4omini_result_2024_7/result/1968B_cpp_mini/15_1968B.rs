use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let xk: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let x = xk[0]; // Not used further in the code
        let k = xk[1]; // Not used further in the code
        
        let a: String = lines.next().unwrap().unwrap();
        let b: String = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(i) {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}