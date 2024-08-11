use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let x = tokens.next().unwrap().parse::<i32>().unwrap();
        let k = tokens.next().unwrap().parse::<i32>().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a = input.trim();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b = input.trim();
        
        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(i) {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}