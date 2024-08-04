use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(Result::unwrap);

    let t: i32 = iterator.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input_line: Vec<i32> = iterator.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let x = input_line[0];
        let k = input_line[1];

        let a: Vec<char> = iterator.next().unwrap().chars().collect();
        let b: Vec<char> = iterator.next().unwrap().chars().collect();

        let mut ans = 0;
        for i in b {
            if let Some(c) = a.get(ans as usize) {
                if *c == i {
                    ans += 1;
                }
            }
        }

        println!("{}", ans);
    }
}