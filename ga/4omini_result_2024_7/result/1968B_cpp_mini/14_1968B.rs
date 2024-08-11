use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let x: i32 = parts.next().unwrap().parse().unwrap(); // x is read but not used
        let k: i32 = parts.next().unwrap().parse().unwrap(); // k is read but not used

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: String = input.trim().to_string();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b: String = input.trim().to_string();

        let mut ans = 0;
        let mut a_chars = a.chars();

        for char_b in b.chars() {
            if let Some(char_a) = a_chars.next() {
                if char_a == char_b {
                    ans += 1;
                } else {
                    // If the current character of `a` does not match, continue checking next
                    a_chars = std::iter::once(char_a).chain(a_chars);
                }
            }
        }
        println!("{}", ans);
    }
}