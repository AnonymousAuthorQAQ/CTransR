use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the number of test cases
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sn = input.trim();
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sm = input.trim();

        // Initialize indices for both strings
        let mut i = 0;  
        let mut j = 0;

        // Iterate to find the longest subsequence in order
        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;
        }

        // Print the result
        println!("{}", i);
    }
}