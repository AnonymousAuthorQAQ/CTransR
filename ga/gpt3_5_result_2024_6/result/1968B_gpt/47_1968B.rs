use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(Result::unwrap);

    let t: i32 = iterator.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<String> = iterator.next().unwrap().split_whitespace().map(String::from).collect();
        let n: usize = input[0].parse().unwrap();
        let m: usize = input[1].parse().unwrap();
        let sn: String = input[2].parse().unwrap();
        let sm: String = input[3].parse().unwrap();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}