use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let m = parts.next().unwrap().parse::<i32>().unwrap();
        let sn = parts.next().unwrap();
        let sm = parts.next().unwrap();

        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);

        t -= 1;
    }
}