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
        let a: String = parts.next().unwrap().to_string();
        let b: String = parts.next().unwrap().to_string();

        let ans = find_longest_prefix(&a, &b);
        println!("{}", ans);
    }
}

fn find_longest_prefix(a: &str, b: &str) -> usize {
    let mut ans = 0;
    let mut iter_b = b.chars();
    for char_a in a.chars() {
        if let Some(char_b) = iter_b.next() {
            if char_a == char_b {
                ans += 1;
            }
        } else {
            break;
        }
    }
    ans
}