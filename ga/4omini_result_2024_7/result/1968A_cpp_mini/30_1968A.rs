use std::io;
use std::cmp::max;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(mid: i64, x: i64) -> bool {
    // The function always returns true since gcd(x, mid) + mid is always positive for mid > 0.
    gcd(x, mid) + mid > 0
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Input not a number");
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let x: i64 = input.trim().parse().expect("Input not a number");

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