use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        
        // Read n, m, sn, and sm
        stdin.read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        
        let sn = parts[2];
        let sm = parts[3];

        let mut i = 0;
        let mut j = 0;

        // Count how many characters from sn can be matched in sm
        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;
        }

        // Output the count of matched characters
        println!("{}", i);
    }
}