use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let x: i32 = tokens.next().unwrap().parse().unwrap();
        let k: i32 = tokens.next().unwrap().parse().unwrap();
        let a: String = tokens.next().unwrap().to_string();
        let b: String = tokens.next().unwrap().to_string();

        let ans = longest_common_prefix(&a, &b);
        println!("{}", ans);
    }
}

fn longest_common_prefix(a: &str, b: &str) -> usize {
    let mut ans = 0;
    for (i, c) in b.chars().enumerate() {
        if ans < a.len() && a.chars().nth(ans) == Some(c) {
            ans += 1;
        }
    }
    ans
}