use std::io::{self, BufRead};

fn main() {
    // Reading input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Reading the number of test cases
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        // Read x and k (though they are not used in the logic)
        let line = lines.next().unwrap().unwrap();
        let mut inputs = line.split_whitespace();
        let x: i32 = inputs.next().unwrap().parse().unwrap();
        let k: i32 = inputs.next().unwrap().parse().unwrap();

        // Read strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        let mut chars_a = a.chars().peekable();

        // Iterate over chars in b and count how many can be matched in order from a
        for char_b in b.chars() {
            while let Some(&char_a) = chars_a.peek() {
                if char_a == char_b {
                    ans += 1;
                    chars_a.next(); // Move to the next char in a
                    break;
                }
                chars_a.next(); // Move to the next char in a if no match
            }
        }

        // Print the result for this test case
        println!("{}", ans);
    }
}