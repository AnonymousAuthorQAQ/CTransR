use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: i64, mid: i64) -> bool {
    gcd(x, mid) + mid
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let x: i64 = lines.next().unwrap().trim().parse().unwrap();
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