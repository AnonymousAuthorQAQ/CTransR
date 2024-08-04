use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // Read number of test cases

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap(); // Read x
        let k: i32 = iter.next().unwrap().parse().unwrap(); // Read k

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a = input.trim(); // Read string a

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let b = input.trim(); // Read string b

        let mut ans = 0;
        for ch in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(ch) {
                ans += 1;
            }
        }
        println!("{}", ans); // Output the result
    }
}