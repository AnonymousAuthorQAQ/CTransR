use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: i32 = lines.next().unwrap().unwrap().parse().unwrap();
    
    while t > 0 {
        let input: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let x = input[0];
        let k = input[1];
        
        let a: String = lines.next().unwrap().unwrap();
        let b: String = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        let mut b_iter = b.chars();
        
        for c in a.chars() {
            if ans < k && c == b_iter.next().unwrap() {
                ans += 1;
            }
        }
        
        println!("{}", ans);
        
        t -= 1;
    }
}