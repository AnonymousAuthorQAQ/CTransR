use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // Read the number of test cases

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut nums = input.split_whitespace();
        let x: i32 = nums.next().unwrap().parse().unwrap(); // Read x
        let k: i32 = nums.next().unwrap().parse().unwrap(); // Read k

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: String = input.trim().to_string(); // Read string a

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b: String = input.trim().to_string(); // Read string b

        let mut ans = 0;

        for char_b in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == char_b as u8 {
                ans += 1;
            }
        }

        println!("{}", ans); // Output the result
    }
}