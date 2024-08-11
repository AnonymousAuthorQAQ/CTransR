use std::io::{self, BufRead};

fn can(x: i64, mid: i64) -> bool {
    x.gcd(&mid) + mid
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: i64 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let x: i64 = lines.next().unwrap().trim().parse().unwrap();

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