use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let x: i32 = tokens.next().unwrap().parse().expect("Please enter a number");
        let k: i32 = tokens.next().unwrap().parse().expect("Please enter a number");
        let a: String = tokens.next().unwrap().to_string();
        let b: String = tokens.next().unwrap().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}