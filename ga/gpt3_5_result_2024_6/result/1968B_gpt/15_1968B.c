use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = iterator.next().unwrap();
        let mut iter = line.split_whitespace();
        
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let m: i32 = iter.next().unwrap().parse().unwrap();
        let sn: String = iter.next().unwrap().parse().unwrap();
        let sm: String = iter.next().unwrap().parse().unwrap();
        
        let (mut i, mut j) = (0, 0);

        while i < n && j < m {
            if sn.chars().nth(i as usize) == sm.chars().nth(j as usize) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}