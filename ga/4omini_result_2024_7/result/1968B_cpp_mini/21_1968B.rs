use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap(); // Read number of test cases

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        
        let _x: i32 = split.next().unwrap().parse().unwrap(); // Read and ignore x
        let _k: i32 = split.next().unwrap().parse().unwrap(); // Read and ignore k

        let a: String = lines.next().unwrap().unwrap(); // Read string a
        let b: String = lines.next().unwrap().unwrap(); // Read string b

        let mut ans = 0;
        let mut a_iter = a.chars();

        // Iterate through characters of b
        for ch in b.chars() {
            // Check if the current character in b matches the current character in a
            if ans < a.len() && Some(ch) == a_iter.next() {
                ans += 1;
            }
        }

        println!("{}", ans); // Print the length of the longest prefix found
    }
}