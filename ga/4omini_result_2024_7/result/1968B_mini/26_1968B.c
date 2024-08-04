use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read n and m
        let nm: Vec<usize> = lines.next().unwrap().unwrap()
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = nm[0];
        let m = nm[1];

        // Read strings sn and sm
        let sn: String = lines.next().unwrap().unwrap();
        let sm: String = lines.next().unwrap().unwrap();

        let mut i = 0; // Pointer for sn
        let mut j = 0; // Pointer for sm

        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1; // Move pointer for sn if there is a match
            }
            j += 1; // Always move pointer for sm
        }

        // Print the number of matched characters
        println!("{}", i);
    }
}