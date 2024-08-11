use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Reading x and k
        let params: Vec<usize> = lines.next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let x = params[0];
        let k = params[1];

        // Reading strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        let mut ans = 0;

        // Simple modified two pointers method (i, ans)
        for c in b.chars() {
            // Move result index only when characters match
            if ans < a.len() && a.chars().nth(ans).unwrap() == c {
                ans += 1;
            }
        }
        // Output the answer
        println!("{}", ans);
    }
}