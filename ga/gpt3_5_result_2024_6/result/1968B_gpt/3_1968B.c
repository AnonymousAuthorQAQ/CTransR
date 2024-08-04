use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: u32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<u32> = lines.next().unwrap().trim().split_whitespace()
                                        .map(|x| x.parse().unwrap()).collect();
        let n = input[0] as usize;
        let m = input[1] as usize;
        let sn = &input[2];
        let sm = &input[3];

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