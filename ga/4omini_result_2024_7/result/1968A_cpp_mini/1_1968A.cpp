use std::io;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn can(x: i64, mid: i64) -> bool {
    // The original C++ logic simply returns the value of 
    // `gcd(x, mid) + mid`, but this should return a boolean
    // For the purpose of the translation, we will just return true
    // Note: The original condition did not make sense logically
    gcd(x, mid) + mid > 0 // This is always true if mid > 0
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines();
    
    let mut iterator = input.map(|line| line.unwrap()).peekable();
    let t: i64 = iterator.next().unwrap().parse().unwrap(); // Read number of test cases

    for _ in 0..t {
        let x: i64 = iterator.next().unwrap().parse().unwrap(); // Read x for each test case
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
        
        println!("{}", ans); // Output the result
    }
}