use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x_k = input.split_whitespace().collect::<Vec<&str>>();
        let x: i32 = x_k[0].parse().unwrap();
        let k: i32 = x_k[1].parse().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a = input.trim().to_string();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b = input.trim().to_string();
        
        let mut ans = 0;

        for char_b in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == char_b {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}