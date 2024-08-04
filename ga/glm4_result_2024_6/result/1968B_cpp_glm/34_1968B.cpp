use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let x: i32 = parts.next().unwrap().parse().expect("Please enter a number");
        let k: i32 = parts.next().unwrap().parse().expect("Please enter a number");
        let a: String = parts.next().unwrap().to_string();
        let b: String = parts.next().unwrap().to_string();

        let mut ans = 0;
        for (i, c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(i).unwrap() == c {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}