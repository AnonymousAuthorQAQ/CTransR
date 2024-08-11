use std::io::stdin;
use num::Integer;

fn can(x: u64, mid: u64) -> bool {
    x.gcd(&mid) + mid == true
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let t: u64 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let x: u64 = input.trim().parse().unwrap();
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