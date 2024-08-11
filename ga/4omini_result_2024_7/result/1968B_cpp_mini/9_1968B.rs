use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        // Read x and k
        let xk: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let x = xk[0];
        let k = xk[1];
        
        // Read strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;

        for char_b in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] as char == char_b {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}