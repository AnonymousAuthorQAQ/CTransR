use std::io;
use std::cmp;
use num::integer::gcd;

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid > 0
}

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();
    
    stdin.read_line(input).unwrap();
    let t: i64 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(input).unwrap();
        let x: i64 = input.trim().parse().unwrap();
        
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