use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let _x: usize = iter.next().unwrap().parse().expect("Please enter a number");
        let _k: usize = iter.next().unwrap().parse().expect("Please enter a number");
        let a: String = iter.next().unwrap().to_string();
        let b: String = iter.next().unwrap().to_string();

        let mut ans = 0;
        for (i, c) in b.chars().enumerate() {
            if i < a.len() && a.chars().nth(i) == Some(c) {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}