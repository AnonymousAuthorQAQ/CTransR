use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: i32 = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<i32> = iterator.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let x = input[0];
        let k = input[1];
        
        let a: Vec<char> = iterator.next().unwrap().chars().collect();
        let b: Vec<char> = iterator.next().unwrap().chars().collect();
        
        let mut ans = 0;
        for i in b {
            if ans < a.len() && a[ans] == i {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}