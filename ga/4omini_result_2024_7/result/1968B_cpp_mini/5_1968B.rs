use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace(); // Prepare iterator over input
        let _x: i32 = iter.next().unwrap().parse().unwrap(); // Read x (not used)
        let _k: i32 = iter.next().unwrap().parse().unwrap(); // Read k (not used)

        // Read strings a and b
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a = input.trim().to_string();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b = input.trim().to_string();

        let mut ans = 0; // Counter for the matching subsequence length
        let mut a_chars = a.chars(); // Create an iterator over `a`

        // For each character in b
        for char_in_b in b.chars() {
            // Try to advance through the iterator of `a` to find matching characters
            while ans < a.len() && a_chars.next() == Some(char_in_b) {
                ans += 1; // Increment `ans` if there is a match
            }
        }
        
        // Output the answer for the current test case
        println!("{}", ans);
    }
}