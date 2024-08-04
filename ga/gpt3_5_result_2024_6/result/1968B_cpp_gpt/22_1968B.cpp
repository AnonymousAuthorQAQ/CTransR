use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = input.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input_parts = input.next().unwrap().trim().split_whitespace();
        let x: usize = input_parts.next().unwrap().parse().unwrap();
        let k: usize = input_parts.next().unwrap().parse().unwrap();
        
        let a: String = input.next().unwrap().trim().to_string();
        let b: String = input.next().unwrap().trim().to_string();
        
        let mut ans = 0;
        for i in b.chars() {
            if let Some(char_a) = a.chars().nth(ans) {
                if char_a == i {
                    ans += 1;
                }
            }
            if ans >= x {
                break;
            }
        }
        println!("{}", ans);
    }
}