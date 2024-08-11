use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        let x: i32 = values.next().unwrap().parse().expect("Invalid input");
        let k: i32 = values.next().unwrap().parse().expect("Invalid input");
        let a: String = values.next().unwrap().to_string();
        let b: String = values.next().unwrap().to_string();

        let mut ans = 0;
        for (i, &ch) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(i).unwrap() == ch {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}