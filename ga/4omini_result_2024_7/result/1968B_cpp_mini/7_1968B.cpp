use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let x = input[0];
        let k = input[1];

        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();

        let mut ans = 0;
        let a_chars: Vec<char> = a.chars().collect();
        
        for i in b.chars() {
            if ans < a_chars.len() && a_chars[ans] == i {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}