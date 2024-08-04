use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let x = params[0]; // `x` is read but not used
        let k = params[1]; // `k` is read but not used
        
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        let mut a_chars = a.chars();
        
        for char_b in b.chars() {
            if let Some(char_a) = a_chars.next() {
                if char_a == char_b {
                    ans += 1;
                } else {
                    // Keep advancing in a until we either find a match or exhaust `a`
                    while let Some(next_char_a) = a_chars.next() {
                        if next_char_a == char_b {
                            ans += 1;
                            break;
                        }
                    }
                }
            }
        }
        println!("{}", ans);
    }
}