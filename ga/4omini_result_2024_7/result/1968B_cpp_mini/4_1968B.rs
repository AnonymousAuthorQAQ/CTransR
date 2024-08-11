use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        // Read x and k (but we won't use x and k)
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let x = first_line[0];
        let k = first_line[1];

        // Read strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();

        let mut ans = 0;
        for char_b in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(char_b) {
                ans += 1;
            }
        }

        // Print the result
        println!("{}", ans);
    }
}