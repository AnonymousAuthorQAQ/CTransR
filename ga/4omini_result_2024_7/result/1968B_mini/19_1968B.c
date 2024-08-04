use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.split_whitespace()
                                     .map(|x| x.parse().unwrap())
                                     .collect();
        let n = nums[0];
        let m = nums[1];
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let sn = input.trim();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let sm = input.trim();

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