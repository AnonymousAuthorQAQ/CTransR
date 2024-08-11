use std::io;
use std::cmp::Ordering;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i64 = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let x: i64 = input.trim().parse().expect("Please type a number!");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;

            if gcd(x, mid) + mid == x {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        println!("{}", ans);
    }
}

fn gcd(a: i64, b: i64) -> i64 {
    match b.cmp(&0) {
        Ordering::Less => gcd(b, a % b),
        Ordering::Equal => a,
        Ordering::Greater => gcd(b, a % b),
    }
}