use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: i32 = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<i32> = iterator.next().unwrap()
                                    .split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();
        let n = input[0];
        let m = input[1];

        let sn = iterator.next().unwrap();
        let sm = iterator.next().unwrap();
        
        let mut i = 0;
        let mut j = 0;
        
        while i < n && j < m {
            if sn.chars().nth(i as usize) == sm.chars().nth(j as usize) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}