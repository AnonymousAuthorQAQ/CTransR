use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(Result::unwrap);

    let t: i32 = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<i32> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = input[0] as usize;
        let m = input[1] as usize;
        let sn = lines.next().unwrap();
        let sm = lines.next().unwrap();

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