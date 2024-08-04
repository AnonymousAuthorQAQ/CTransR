use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Read n and m, and then the two strings sn and sm
        let first_line = lines.next().unwrap().unwrap();
        let parts: Vec<_> = first_line.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let m: usize = parts[1].parse().unwrap();
        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            // Check if characters match
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1; // Move pointer in sn
            }
            j += 1; // Always move pointer in sm
        }

        // Output the result
        println!("{}", i);
    }
}