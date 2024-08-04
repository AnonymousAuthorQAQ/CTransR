use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let x = input[0];
        let k = input[1];

        let a = lines.next().unwrap();
        let b = lines.next().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if let Some(ch) = a.chars().nth(ans) {
                if ch == i {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}