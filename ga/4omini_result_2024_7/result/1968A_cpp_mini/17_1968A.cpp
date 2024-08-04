use std::io;
use std::cmp::max;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: i64, mid: i64) -> bool {
    // The original code logic suggests we check if gcd(x, mid) + mid is greater than or equal to mid
    // This condition is always satisfied due to its formulation:
    // Instead of checking it we just return true.
    // But the intention seems to be checking the condition as it might have been needed.
    gcd(x, mid) + mid >= mid
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: i64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();

        let mut l = 1;
        let mut r = x - 1;
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