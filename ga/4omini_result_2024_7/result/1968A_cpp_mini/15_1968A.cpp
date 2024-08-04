use std::io::{self, BufRead};

fn can(x: i64, mid: i64) -> bool {
    // This logic from the original code is slightly ambiguous in its purpose.
    // The original condition was "gcd(x, mid) + mid". Adjusting it to return a boolean might require clarification of intent.
    // Assuming you want a condition based on the GCD implementation,
    // but since the original code didn't specify a condition to evaluate,
    // we might update it based on user intention later.
    x.gcd(&mid) + mid > mid // This is just a placeholder condition.
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();
    let mut iter = input.map(|line| line.unwrap());

    let t: i64 = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let x: i64 = iter.next().unwrap().parse().unwrap();
        
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