use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let n: i32 = tokens.next().unwrap().parse().expect("Please enter a number");
        let m: i32 = tokens.next().unwrap().parse().expect("Please enter a number");
        let sn: String = tokens.next().unwrap().to_string();
        let sm: String = tokens.next().unwrap().to_string();

        let mut i = 0;
        let mut j = 0;
        while i < n as usize && j < m as usize {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}