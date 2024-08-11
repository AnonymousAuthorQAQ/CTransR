use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = iterator.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<i32> = iterator.next().unwrap()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect();

        let x = input[0];
        let k = input[1];

        let a = iterator.next().unwrap();
        let b = iterator.next().unwrap();

        let mut ans = 0;
        for (idx, c) in b.chars().enumerate() {
            if let Some(bc) = a.chars().nth(idx) {
                if bc == c {
                    ans += 1;
                }
            }
        }

        println!("{}", ans);
    }
}