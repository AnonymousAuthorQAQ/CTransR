use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().expect("Invalid input");
        let m: usize = parts.next().unwrap().parse().expect("Invalid input");
        let sn: String = parts.next().unwrap().to_string();
        let sm: String = parts.next().unwrap().to_string();

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