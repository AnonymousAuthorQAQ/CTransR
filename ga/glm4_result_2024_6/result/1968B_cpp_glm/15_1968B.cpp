use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        let _x: usize = values.next().unwrap().parse().expect("Please enter a number");
        let _k: usize = values.next().unwrap().parse().expect("Please enter a number");

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        let a: String = values.next().unwrap().to_string();
        let b: String = values.next().unwrap().to_string();

        let mut ans = 0;
        for (i, char_b) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(ans) == Some(char_b) {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}