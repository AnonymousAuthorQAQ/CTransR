use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a = input.trim();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b = input.trim();

        let mut ans = 0;
        for (i, val) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == val {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}