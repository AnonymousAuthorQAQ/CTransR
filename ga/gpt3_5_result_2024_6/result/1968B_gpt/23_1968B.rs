use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Error reading line");
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().expect("Invalid input");
        let m: usize = parts.next().unwrap().parse().expect("Invalid input");

        input.clear();
        io::stdin().read_line(&mut input).expect("Error reading line");
        let sn = input.trim();

        input.clear();
        io::stdin().read_line(&mut input).expect("Error reading line");
        let sm = input.trim();

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