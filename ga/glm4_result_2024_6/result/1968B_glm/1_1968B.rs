use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().expect("Missing n").parse().expect("Please enter a number");
        let m: usize = parts.next().expect("Missing m").parse().expect("Please enter a number");
        let sn = parts.next().expect("Missing sn").to_string();
        let sm = parts.next().expect("Missing sm").to_string();

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