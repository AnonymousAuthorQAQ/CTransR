use std::io::{self, Write};
use std::cmp::Ordering;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid > 0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let x: u64 = input.trim().parse().unwrap();

        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;
            match can(x, mid) {
                true => {
                    ans = mid;
                    l = mid + 1;
                },
                false => {
                    r = mid - 1;
                },
            }
        }
        let mut stdout = io::stdout();
        writeln!(stdout, "{}", ans).unwrap();
    }
}