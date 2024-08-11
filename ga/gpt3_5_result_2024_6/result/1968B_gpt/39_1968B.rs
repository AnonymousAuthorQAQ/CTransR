use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

     let t: i32 = iterator.next().unwrap().parse().unwrap();
     for _ in 0..t {
        let input_line: String = iterator.next().unwrap();
        let input_vec: Vec<&str> = input_line.trim().split_whitespace().collect();
        let n: usize = input_vec[0].parse().unwrap();
        let m: usize = input_vec[1].parse().unwrap();
        let sn: &str = input_vec[2];
        let sm: &str = input_vec[3];
        
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