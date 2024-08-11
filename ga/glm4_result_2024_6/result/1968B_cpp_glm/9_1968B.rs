use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let x: i32 = parts.next().unwrap().parse().expect("Please enter a number");
        let k: i32 = parts.next().unwrap().parse().expect("Please enter a number");
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: String = input.trim().to_string();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let b: String = input.trim().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(i) {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}