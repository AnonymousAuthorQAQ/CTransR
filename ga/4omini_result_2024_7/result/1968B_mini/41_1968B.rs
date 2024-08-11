use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap(); // Read the number of test cases
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap(); // Read n and m
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();

        input.clear();
        stdin.read_line(&mut input).unwrap(); // Read the strings sn and sm
        let sn = input.trim().to_string();
        
        input.clear();
        stdin.read_line(&mut input).unwrap(); // Read the next string sm
        let sm = input.trim().to_string();

        let mut i = 0;
        let mut j = 0;

        // Main logic to find the longest matched prefix
        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1; // Move to the next character in sn
            }
            j += 1; // Always move to the next character in sm
        }

        println!("{}", i);
    }
}