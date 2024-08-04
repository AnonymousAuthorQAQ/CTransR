use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        let x = first_line[0];
        let k = first_line[1];
        
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == i as u8 {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}