use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input.split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let x = parts[0];
        let k = parts[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut token = input.split_whitespace();
        let a = token.next().unwrap();
        let b = token.next().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if let Some(ch) = a.chars().nth(ans as usize) {
                if ch == i {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}