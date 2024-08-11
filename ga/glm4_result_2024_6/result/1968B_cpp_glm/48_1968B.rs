use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let _x: i32 = tokens.next().unwrap().parse().unwrap();
        let _k: i32 = tokens.next().unwrap().parse().unwrap();
        let a: String = tokens.next().unwrap().to_string();
        let b: String = tokens.next().unwrap().to_string();

        let mut ans = 0;
        for (i, c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(ans) == Some(c) {
                ans += 1;
            } else {
                break;
            }
        }

        println!("{}", ans);
    }
}