use std::io;

fn main() {
    // Allow the use of buffered I/O for performance
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // Read the number of test cases

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let x: i32 = parts[0].parse().unwrap(); // Reading x
        let k: i32 = parts[1].parse().unwrap(); // Reading k

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a = input.trim(); // Reading string a

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b = input.trim(); // Reading string b

        let mut ans = 0; // Initialize ans
        let a_chars: Vec<char> = a.chars().collect(); // Create a char vector from string a

        // Iterate through each character in string b
        for i in b.chars() {
            if ans < a_chars.len() && a_chars[ans] == i {
                ans += 1; // Increment ans if characters match
            }
        }
        
        // Print the result for this test case
        println!("{}", ans);
    }
}