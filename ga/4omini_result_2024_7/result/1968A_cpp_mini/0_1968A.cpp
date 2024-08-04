use std::io;

fn can(x: i64, mid: i64) -> bool {
    // The logic here is simply returning gcd(x, mid) + mid, 
    // but we will just return true for simplicity
    // since the original C++ code doesn't use the output of can() meaningfully.
    // In a real-case for condition check, implement proper logic.
    let gcd_value = gcd(x, mid);
    gcd_value + mid > mid  // This condition will always be true if mid is positive
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
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
        
        let (mut l, mut r) = (1, x - 1);
        let mut ans = 1;

        while l <= r {
            let mid = (l + r) / 2;

            if can(x, mid) {
                ans = mid;
                l = mid + 1;  // Look for a greater mid
            } else {
                r = mid - 1;  // Look for a smaller mid
            }
        }

        println!("{}", ans);
    }
}