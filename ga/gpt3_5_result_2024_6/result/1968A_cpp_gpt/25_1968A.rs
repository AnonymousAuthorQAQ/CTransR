use std::io::{self, BufRead};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) == 1
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: u32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let x: u64 = lines.next().unwrap().trim().parse().unwrap();
        
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;
        
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