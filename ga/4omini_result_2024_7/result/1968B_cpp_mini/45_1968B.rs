use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        let x: i32 = values.next().unwrap().parse().unwrap();
        let k: i32 = values.next().unwrap().parse().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a = input.trim().to_string();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b = input.trim().to_string();

        let mut ans = 0;
        for char_b in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] as char == char_b {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}