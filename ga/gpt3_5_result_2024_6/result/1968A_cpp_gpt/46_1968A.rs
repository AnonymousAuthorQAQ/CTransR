use std::io::{self, prelude::*};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: u64 = input.trim().parse().unwrap();
        
        let mut l: u64 = 1;
        let mut r: u64 = x - 1;
        let mut ans: u64 = 1;
        
        while l <= r {
            let mid = (l + r) / 2;
            if can(x, mid) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        
        println!("{}", ans);
    }
}

fn can(x: u64, mid: u64) -> bool {
    // implement the gcd logic
    unimplemented!()
}