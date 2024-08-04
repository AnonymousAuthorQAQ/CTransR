use std::io::{self, BufRead};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }
    a
}

fn can(x: u64, mid: u64) -> bool {
    gcd(x, mid) + mid < x
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: u64 = iterator.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let x: u64 = iterator.next().unwrap().trim().parse().unwrap();
        let mut l: u64 = 1;
        let mut r: u64 = x - 1;
        let mut ans: u64 = 1;

        while l <= r {
            let mid: u64 = (l + r) / 2;
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