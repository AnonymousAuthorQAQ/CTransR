use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();
    let mut lines = input.map(|line| line.unwrap()).collect::<Vec<String>>();
    
    let t: usize = lines.remove(0).parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.remove(0).split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        
        let x = first_line[0]; // x is read but not used
        let k = first_line[1]; // k is read but not used
        
        let a = lines.remove(0).clone();
        let b = lines.remove(0).clone();
        
        let mut ans = 0;
        let a_chars: Vec<char> = a.chars().collect();
        
        for i in b.chars() {
            if ans < a_chars.len() && a_chars[ans] == i {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}