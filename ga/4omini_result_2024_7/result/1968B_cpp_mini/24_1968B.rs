use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: String = input.trim().to_string();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b: String = input.trim().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == i as u8 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}