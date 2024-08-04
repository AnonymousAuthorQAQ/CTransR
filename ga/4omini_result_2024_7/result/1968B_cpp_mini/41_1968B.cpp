use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: String = input.trim().to_string();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b: String = input.trim().to_string();

        let mut ans = 0;
        for char_b in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == char_b as u8 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}