use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap(); // Read the number of test cases

    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = first_line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap(); // Length of sn
        let m: usize = parts[1].parse().unwrap(); // Length of sm

        let sn = lines.next().unwrap().unwrap(); // Read string sn
        let sm = lines.next().unwrap().unwrap(); // Read string sm

        let mut i = 0; // Index for sn
        let mut j = 0; // Index for sm

        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1; // Move to the next character in sn if there's a match
            }
            j += 1; // Always move to the next character in sm
        }

        println!("{}", i); // Output the length of the longest subsequence found
    }
}