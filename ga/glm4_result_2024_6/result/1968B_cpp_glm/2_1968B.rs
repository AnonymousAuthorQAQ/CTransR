use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let _x: i32 = parts.next().unwrap().parse().expect("Please enter a number");
        let _k: i32 = parts.next().unwrap().parse().expect("Please enter a number");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: String = input.trim().to_string();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let b: String = input.trim().to_string();

        let ans = find_longest_prefix(&a, &b);
        println!("{}", ans);
    }
}

fn find_longest_prefix(a: &str, b: &str) -> usize {
    let mut ans = 0;
    for (i, c) in b.chars().enumerate() {
        if ans < a.len() && a.chars().nth(i) == Some(c) {
            ans += 1;
        }
    }
    ans
}