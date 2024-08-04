use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read x and k (currently unused)
        let xk: Vec<i32> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let x = xk[0]; // x is unused
        let k = xk[1]; // k is unused

        // Read strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();

        // Find the length of the longest prefix of b that can be formed by characters of a in order
        let mut ans = 0;
        let mut a_chars = a.chars().peekable();

        for i in b.chars() {
            if ans < a.len() && a_chars.peek() == Some(&i) {
                a_chars.next(); // Move to the next character in a
                ans += 1; // Increment the match count
            }
        }

        // Output the result
        println!("{}", ans);
    }
}