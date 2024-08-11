use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<i32> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let x = input[0];
        let k = input[1];
        
        let a: String = lines.next().unwrap();
        let b: String = lines.next().unwrap();
        
        let mut ans = 0;
        for c in b.chars() {
            if let Some(d) = a.chars().nth(ans) {
                if c == d {
                    ans += 1;
                }
            }
        }
        
        println!("{}", ans);
    }
}