use std::io::{self, Write};
use std::num::NonZeroUsize;

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(mid: u64, x: u64) -> bool {
    gcd(x, mid) + mid > 0 // Always true as both parts are positive
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    let mut output = String::new();

    // Process each test case
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        
        let x: u64 = input.trim().parse().unwrap();
        
        // Binary search for the largest valid mid
        let mut l = 1;
        let mut r = x - 1;
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

        output.push_str(&format!("{}\n", ans));
    }

    // Display all results
    print!("{}", output);
}