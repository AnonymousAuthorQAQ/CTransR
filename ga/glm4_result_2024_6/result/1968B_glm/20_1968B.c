use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid test case count");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut tokens = input.trim().split_whitespace();
        let n: i32 = tokens.next().expect("Invalid n").parse().expect("Invalid n");
        let m: i32 = tokens.next().expect("Invalid m").parse().expect("Invalid m");
        let sn: String = tokens.next().expect("Invalid sn").to_string();
        let sm: String = tokens.next().expect("Invalid sm").to_string();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sn.chars().nth(i as usize).expect("sn index out of bounds") == sm.chars().nth(j as usize).expect("sm index out of bounds") {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}