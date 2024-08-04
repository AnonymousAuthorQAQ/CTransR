use std::io::{self, Write};
use std::cmp::Ordering;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: u64, mid: u64) -> u64 {
    gcd(x, mid) + mid
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _i in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: u64 = input.trim().parse().unwrap();

        let mut l: u64 = 1;
        let mut r: u64 = x - 1;
        let mut ans: u64 = 1;

        while l <= r {
            let mid: u64 = (l + r) / 2;
            match can(x, mid).cmp(&0) {
                Ordering::Greater => {
                    ans = mid;
                    l = mid + 1;
                }
                _ => {
                    r = mid - 1;
                }
            }
        }

        println!("{}", ans);
        let _ = io::stdout().flush();
    }
}