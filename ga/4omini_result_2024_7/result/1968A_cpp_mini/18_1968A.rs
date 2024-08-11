use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(mid: i64, x: i64) -> bool {
    gcd(x, mid) + mid > 0
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut lines = handle.lines();
    let t: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let x: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        let (mut l, mut r) = (1, x - 1);
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