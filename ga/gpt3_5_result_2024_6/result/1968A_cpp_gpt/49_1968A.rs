use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let x: i64 = lines.next().unwrap().trim().parse().unwrap();
        let mut l = 1;
        let mut r = x - 1;
        let mut ans = 1;
        
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

fn gcd(a: i64, b: i64) -> bool {
    // Implement the gcd function here
    // Example implementation is shown below
    /*let mut a = a.abs();
    let mut b = b.abs();
    while b != i64::default() {
        let r = a % b;
        a = b;
        b = r;
    }
    a*/
    // Placeholder return value, should be replaced with actual implementation
    true
}