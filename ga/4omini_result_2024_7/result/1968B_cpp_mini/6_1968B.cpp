use std::io;

fn main() {
    // Use buffered reader to read input more efficiently
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear(); // Clear input buffer for next test case
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        
        let x: i32 = parts.next().unwrap().parse().unwrap(); // Read x (not used)
        let k: i32 = parts.next().unwrap().parse().unwrap(); // Read k (not used)

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: String = input.trim().to_string();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b: String = input.trim().to_string();

        let mut ans = 0;
        let mut a_chars = a.chars();
        
        for i in b.chars() {
            if let Some(a_char) = a_chars.next() {
                if a_char == i {
                    ans += 1;
                } else {
                    // If current character in a doesn't match, we ignore it and move on
                    // since we are looking for a prefix matched in order.
                    continue;
                }
            }
        }

        println!("{}", ans);
    }
}