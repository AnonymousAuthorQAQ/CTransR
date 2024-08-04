use std::io;
use std::cmp::Ordering;
use std::ops::Sub;

fn can(x: i64, mid: i64) -> bool {
    let gcd = x.gcd(&mid);
    gcd + mid > mid // Assuming the condition is that gcd(x, mid) + mid > mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: i64 = input.trim().parse().expect("Please enter a number");

        let mut l = 1;
        let mut r = x.sub(1);
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