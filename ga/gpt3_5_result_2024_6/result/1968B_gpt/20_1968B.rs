use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut line = String::new();

    stdin.lock().read_line(&mut line).unwrap();
    let t: i32 = line.trim().parse().unwrap();

    for _ in 0..t {
        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        let mut parts = line.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();

        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        let sn = line.trim();

        line.clear();
        stdin.lock().read_line(&mut line).unwrap();
        let sm = line.trim();

        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}