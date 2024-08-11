use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let x = input[0];
        let k = input[1];

        let a: String = lines.next().unwrap();
        let b: String = lines.next().unwrap();
        
        let mut ans = 0;
        for (i, character) in b.chars().enumerate() {
            if i < x && i < a.len() && a.chars().nth(i).unwrap() == character {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}