use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = iterator.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input_line: Vec<i32> = iterator.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let x = input_line[0];
        let k = input_line[1];

        let a: String = iterator.next().unwrap();
        let b: String = iterator.next().unwrap();

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