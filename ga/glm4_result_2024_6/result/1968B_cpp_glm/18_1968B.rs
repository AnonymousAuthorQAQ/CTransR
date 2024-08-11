use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let _x: i32 = values.next().unwrap().parse().unwrap(); // Unused
        let _k: i32 = values.next().unwrap().parse().unwrap(); // Unused

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let a: String = values.next().unwrap().to_string();
        let b: String = values.next().unwrap().to_string();

        let mut ans = 0;
        for (i, c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(a.len() - 1 - i).unwrap() == c {
                ans += 1;
            } else {
                break;
            }
        }
        println!("{}", ans);
    }
}