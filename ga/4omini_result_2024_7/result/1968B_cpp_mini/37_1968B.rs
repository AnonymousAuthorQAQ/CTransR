use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let _x: usize = parts.next().unwrap().parse().unwrap(); // x is read but not used
        let _k: usize = parts.next().unwrap().parse().unwrap(); // k is read but not used

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a = input.trim().to_string();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b = input.trim().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(i) {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}