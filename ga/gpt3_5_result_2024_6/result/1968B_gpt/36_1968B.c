use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: u32 = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input: Vec<u32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input[0] as usize;
        let m = input[1] as usize;

        let sn = lines.next().unwrap().trim();
        let sm = lines.next().unwrap().trim();

        let (mut i, mut j) = (0, 0);

        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}