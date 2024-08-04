use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|l| l.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap().trim().split(" ").map(|x| x.parse().unwrap()).collect();
        let x = input[0];
        let k = input[1];
        
        let a: String = lines.next().unwrap().trim().to_string();
        let b: String = lines.next().unwrap().trim().to_string();
        
        let mut ans = 0;
        for c in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == c {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}