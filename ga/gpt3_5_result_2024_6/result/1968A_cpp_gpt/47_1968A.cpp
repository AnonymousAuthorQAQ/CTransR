use std::io::{self, Read};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn can(x: u64, mid: u64) -> bool {
    return gcd(x, mid) + mid == 1;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.trim().split('\n');
    let t: u64 = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let x: u64 = lines.next().unwrap().trim().parse().unwrap();

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