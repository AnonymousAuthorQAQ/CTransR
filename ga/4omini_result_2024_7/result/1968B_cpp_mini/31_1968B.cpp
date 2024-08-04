use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let (x, k) = (params[0], params[1]);
        
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        let a_chars: Vec<char> = a.chars().collect();
        
        for b_char in b.chars() {
            if ans < a_chars.len() && a_chars[ans] == b_char {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}