use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let _x: usize = tokens.next().unwrap().parse().expect("Invalid input"); // Unused variable
        let _k: usize = tokens.next().unwrap().parse().expect("Invalid input"); // Unused variable
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();
        println!("{}", longest_matching_prefix(a, b));
    }
}

fn longest_matching_prefix<'a>(a: &'a str, b: &'a str) -> usize {
    let mut ans = 0;
    for (i, c) in b.chars().enumerate() {
        if ans < a.len() && a.chars().nth(ans) == Some(c) {
            ans += 1;
        }
    }
    ans
}