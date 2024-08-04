use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: i64 = iterator.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let x: i64 = iterator.next().unwrap().parse().unwrap();

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

fn can(x: i64, mid: i64) -> bool {
    x.gcd(&mid).wrapping_add(mid) > 0
}