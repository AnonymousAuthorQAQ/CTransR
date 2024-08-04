use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let x: u64 = iterator.next().unwrap().trim().parse().unwrap();
        let mut l: u64 = 1;
        let mut r: u64 = x - 1;
        let mut ans: u64 = 1;

        while l <= r {
            let mid = (l + r) / 2;
            if gcd(x, mid) + mid {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }

        println!("{}", ans);
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}