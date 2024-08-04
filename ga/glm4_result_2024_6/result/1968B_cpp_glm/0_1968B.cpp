use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let (x, k) = (line.parse::<usize>().unwrap(), line.parse::<usize>().unwrap());
        line = lines.next().unwrap().unwrap();
        let (a, b) = line.trim().split_at(x);
        let a: String = a.chars().collect();
        let b: String = b.chars().collect();

        let mut ans = 0;
        for (i, c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(ans) == Some(c) {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}