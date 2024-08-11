use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let x: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let k: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let a: String = tokens.next().unwrap().to_string();
        let b: String = tokens.next().unwrap().to_string();

        let mut ans = 0;
        for (i, c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(i).unwrap() == c {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}