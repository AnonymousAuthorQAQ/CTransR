use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid == 0
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: i64 = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let x: i64 = iterator.next().unwrap().parse().unwrap();
        let mut l: i64 = 1;
        let mut r: i64 = x - 1;
        let mut ans: i64 = 1;
        
        while l <= r {
            let mid: i64 = (l + r) / 2;
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