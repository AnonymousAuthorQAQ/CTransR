use std::io::{self, Read};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut params = lines.next().unwrap().split_whitespace();
        let x: i32 = params.next().unwrap().parse().unwrap();
        let k: i32 = params.next().unwrap().parse().unwrap();
        
        let a = lines.next().unwrap().to_string();
        let b = lines.next().unwrap().to_string();
        
        let mut ans = 0;
        for ch in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == ch as u8 {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}