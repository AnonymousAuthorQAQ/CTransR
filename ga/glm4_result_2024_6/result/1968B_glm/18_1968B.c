use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let n: usize = tokens.next().expect("Please enter a number").parse().expect("Please enter a number");
        let m: usize = tokens.next().expect("Please enter a number").parse().expect("Please enter a number");
        let sn: String = tokens.next().expect("Please enter a string").to_string();
        let sm: String = tokens.next().expect("Please enter a string").to_string();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}