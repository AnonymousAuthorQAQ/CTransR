use std::io::{self, prelude::*};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(Result::unwrap);

    let t: i32 = iterator.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let inputs: Vec<i32> = iterator.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let x = inputs[0];
        let k = inputs[1];
        let a = &iterator.next().unwrap();
        let b = &iterator.next().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}