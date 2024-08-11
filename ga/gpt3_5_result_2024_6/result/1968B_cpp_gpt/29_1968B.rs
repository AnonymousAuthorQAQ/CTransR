use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input: Vec<usize> = iterator.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let x = input[0];
        let k = input[1];

        let a = iterator.next().unwrap();
        let b = iterator.next().unwrap();

        let mut ans = 0;
        for (i, j) in a.chars().zip(b.chars()).enumerate() {
            if i >= x { break; }

            if j.0 == j.1 {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}