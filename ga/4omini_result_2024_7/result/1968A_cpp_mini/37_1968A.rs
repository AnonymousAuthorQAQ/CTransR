use std::io;

fn can(x: i64, mid: i64) -> bool {
    // This function is effectively always true because gcd is always non-negative.
    (x.gcd(mid) + mid) > 0
}

fn main() {
    let stdin = io::stdin();
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
                l = mid + 1; // Move the left pointer to search for a larger valid 'mid'
            } else {
                r = mid - 1; // Move the right pointer to search for a smaller 'mid'
            }
        }
        
        println!("{}", ans);
    }
}

// Note: In Rust, we need to add an external crate for `gcd` functionality since it is not available in the standard library.
// To do this, add `num` crate to your Cargo.toml file:
// [dependencies]
// num = "0.4"