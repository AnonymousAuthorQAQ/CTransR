use std::io::{self, BufRead};

fn can(x: i64, mid: i64) -> bool {
    num::integer::gcd(x, mid) + mid == 0
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());
    
    let t: i64 = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let x: i64 = iterator.next().unwrap().parse().unwrap();
        let (mut l, mut r) = (1, x - 1);
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