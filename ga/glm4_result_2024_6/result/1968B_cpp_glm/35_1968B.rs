use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let tokens: Vec<&str> = input.trim().split_whitespace().collect();
        let _x: usize = tokens[0].parse().expect("Invalid input"); // Unused variable
        let _k: usize = tokens[1].parse().expect("Invalid input"); // Unused variable

        io::stdin().read_line(&mut input).expect("Failed to read input");
        let a: String = input.trim().to_string();

        io::stdin().read_line(&mut input).expect("Failed to read input");
        let b: String = input.trim().to_string();

        let ans = longest_matching_prefix(&a, &b);
        println!("{}", ans);
    }
}

fn longest_matching_prefix(a: &str, b: &str) -> usize {
    let mut ans = 0;
    for (i, c) in b.chars().enumerate() {
        if ans < a.len() && a.chars().nth(ans) == Some(c) {
            ans += 1;
        } else {
            break;
        }
    }
    ans
}