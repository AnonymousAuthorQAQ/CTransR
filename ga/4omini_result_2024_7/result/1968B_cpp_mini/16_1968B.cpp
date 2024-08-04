use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        // Read x and k
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        
        input.clear();
        // Read strings a and b
        stdin.read_line(&mut input).unwrap();
        let a = input.trim().split_whitespace().nth(0).unwrap();
        let b = input.trim().split_whitespace().nth(1).unwrap();
        
        // Initialize ans variable
        let mut ans = 0;
        // Count the matched characters in order
        for char in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == char as u8 {
                ans += 1;
            }
        }
        // Output the answer for the test case
        println!("{}", ans);
    }
}