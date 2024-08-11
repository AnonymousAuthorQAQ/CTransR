use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: usize = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<usize> = iterator.next().unwrap().split_whitespace()
                                .map(|x| x.parse().unwrap()).collect();
        let x = input[0];
        let k = input[1];
        
        let a: String = iterator.next().unwrap();
        let b: String = iterator.next().unwrap();
        
        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}