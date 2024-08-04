use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input_params: Vec<i32> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let x = input_params[0];
        let k = input_params[1];

        let a: String = lines.next().unwrap();
        let b: String = lines.next().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if let Some(c) = a.chars().nth(ans as usize) {
                if c == i {
                    ans += 1;
                }
            }
        }

        println!("{}", ans);
    }
}