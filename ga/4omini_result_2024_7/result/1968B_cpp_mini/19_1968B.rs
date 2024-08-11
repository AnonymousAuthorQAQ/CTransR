use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        // Read x and k
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let _x: usize = iter.next().unwrap().parse().unwrap();
        let _k: usize = iter.next().unwrap().parse().unwrap();
        
        // Read strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;

        // Determine how many characters of a can be found sequentially in b
        for char_b in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(char_b) {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}