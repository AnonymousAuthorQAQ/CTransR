use std::io;

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
    // Fast input/output
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: i64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();
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