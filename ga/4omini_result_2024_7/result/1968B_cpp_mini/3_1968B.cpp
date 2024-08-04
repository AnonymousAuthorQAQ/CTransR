use std::io::{self, Read};

fn main() {
    let input = read_input();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let x: usize = iter.next().unwrap().parse().unwrap(); // Read but not used
        let k: usize = iter.next().unwrap().parse().unwrap(); // Read but not used
        let a = iter.next().unwrap(); // Read string a
        let b = iter.next().unwrap(); // Read string b
        
        let mut ans = 0;
        let mut a_chars = a.chars();

        for char_b in b.chars() {
            if ans < a.len() && a_chars.clone().nth(ans) == Some(char_b) {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    input
}