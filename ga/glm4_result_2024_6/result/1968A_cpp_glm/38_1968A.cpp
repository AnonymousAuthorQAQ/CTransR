use std::io;
use std::str::FromStr;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let x: u64 = input.trim().parse().unwrap();

        let ans = find_max_gcd_factor(x);
        println!("{}", ans);
    }
}

fn find_max_gcd_factor(x: u64) -> u64 {
    let mut l = 1;
    let mut r = x - 1;
    let mut ans = 1;

    while l <= r {
        let mid = (l + r) / 2;

        if gcd(x, mid) >= mid {
            ans = mid;
            l = mid + 1;
        } else {
            r = mid - 1;
        }
    }

    ans
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}