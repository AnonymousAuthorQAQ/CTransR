use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut params = lines.next().unwrap().split_whitespace();
        let x: i32 = params.next().unwrap().parse().unwrap();
        let k: i32 = params.next().unwrap().parse().unwrap();
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();

        let mut ans = 0;
        for char_b in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(char_b) {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}