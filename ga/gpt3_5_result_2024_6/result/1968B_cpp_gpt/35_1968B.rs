use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = iterator.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = iterator.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let x = input[0];
        let k = input[1];

        let a: String = iterator.next().unwrap().trim().to_string();
        let b: String = iterator.next().unwrap().trim().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}