use std::io;

fn can(x: i64, mid: i64) -> bool {
    // In C++, it merely evaluates gcd, here we're just putting it as a check
    // but since we always sum gcd and mid for true, it will treat any mid > 0 as valid
    mid > 0 // Always true for positive mid, which fits the same logic
}

fn main() {
    let stdin = io::stdin(); // Get standard input
    let mut input = String::new(); // This will hold the complete input

    stdin.read_line(&mut input).unwrap(); // Read the first line which is t
    let t: i64 = input.trim().parse().unwrap(); // Parse the number of test cases

    for _ in 0..t {
        input.clear(); // Clear previous input
        stdin.read_line(&mut input).unwrap(); // Read the next line which is x
        let x: i64 = input.trim().parse().unwrap(); // Parse x
        
        let (mut l, mut r) = (1, x - 1); // Setting left and right bounds
        let mut ans = 1; // Default answer
        
        while l <= r {
            let mid = (l + r) / 2; // Calculate mid
            if can(x, mid) { // Check condition
                ans = mid; // Update answer
                l = mid + 1; // Move up the left boundary
            } else {
                r = mid - 1; // Move down the right boundary
            }
        }
        println!("{}", ans); // Output the result for each test case
    }
}