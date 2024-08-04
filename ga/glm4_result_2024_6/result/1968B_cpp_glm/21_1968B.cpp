use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let x: usize = parts[0].parse().unwrap();
        let k: usize = parts[1].parse().unwrap();
        let a: String = parts[2].to_string();
        let b: String = parts[3].to_string();

        let ans = get_longest_prefix_match(a.as_str(), b.as_str());
        println!("{}", ans);
    }
}

fn get_longest_prefix_match(a: &str, b: &str) -> usize {
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