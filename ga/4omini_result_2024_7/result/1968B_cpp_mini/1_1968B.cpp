use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // number of test cases
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        
        let _x: i32 = parts.next().unwrap().parse().unwrap(); // read x (unused)
        let _k: i32 = parts.next().unwrap().parse().unwrap(); // read k (unused)
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a = input.trim(); // read string a
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b = input.trim(); // read string b
        
        let mut ans = 0;
        let mut a_chars = a.chars().peekable();
        
        for b_char in b.chars() {
            if let Some(&a_char) = a_chars.peek() {
                if a_char == b_char {
                    ans += 1;
                    a_chars.next(); // Move the iterator forward
                }
            }
        }
        
        println!("{}", ans);
    }
}