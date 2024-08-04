use std::io;
use std::cmp::Ordering;
use std::num::gcd;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: u32 = input.trim().parse().expect("Please type a number!");

    for _ in 0..t {
        let mut x_input = String::new();
        io::stdin().read_line(&mut x_input).expect("Failed to read line");
        let x: u64 = x_input.trim().parse().expect("Please type a number!");

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 0;

        while l <= r {
            let mid = l + (r - l) / 2;

            if gcd(x, mid) + mid > x {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        println!("{}", ans);
    }
}