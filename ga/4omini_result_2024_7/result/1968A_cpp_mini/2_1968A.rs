use std::io;

fn main() {
    // Set up input and output
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut input = String::new();
    let mut output = String::new();

    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear(); // Clear input buffer for next test case
        stdin.read_line(&mut input).unwrap();
        let x: i64 = input.trim().parse().unwrap();
        
        let (mut l, mut r) = (1, x - 1);
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
    
    // Output the result once
    stdout.lock().write_all(output.as_bytes()).unwrap();
}

// Function to check the condition as per the original C++ 'can' function
fn can(mid: i64, x: i64) -> bool {
    (gcd(x, mid) + mid) != 0 // Ensured a check against a logical condition
}

// Function to calculate GCD using Euclid's algorithm
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}