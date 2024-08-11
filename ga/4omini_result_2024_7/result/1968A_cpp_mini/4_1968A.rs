use std::io::{self, BufRead};
use std::cmp::max;
use num::integer::gcd;

fn can(mid: i64, x: i64) -> bool {
    // The original C++ code doesn't really use this correctly; we assumed it means 
    // return the value of gcd(x, mid) + mid for some condition. 
    // We reinterpret `can` by returning true if gcd(x, mid) is greater than 0.
    gcd(x, mid) + mid > 0
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines();
    
    let mut t: i64 = reader.next().unwrap().unwrap().parse().unwrap();
    
    while t > 0 {
        t -= 1;
        let x: i64 = reader.next().unwrap().unwrap().parse().unwrap();
        
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;
        
        while l <= r {
            let mid = (l + r) / 2;
            if can(mid, x) {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        println!("{}", ans);
    }
}