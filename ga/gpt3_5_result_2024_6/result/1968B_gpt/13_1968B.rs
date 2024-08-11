use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input_lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: i32 = input_lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = input_lines.next().unwrap();
        let parts: Vec<&str> = line.split(' ').collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        let sn: &str = parts[2];
        let sm: &str = parts[3];
        
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