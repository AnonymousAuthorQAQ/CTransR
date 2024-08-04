use std::io;
use std::str::FromStr;

fn main() {
    let t = read_int();

    for _ in 0..t {
        let x = read_int();
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

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid == x
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn read_int() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i64>().unwrap()
}