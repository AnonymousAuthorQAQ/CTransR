use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Invalid input");
        let m: usize = iter.next().unwrap().parse().expect("Invalid input");
        let sn: String = iter.next().unwrap().parse().expect("Invalid input");
        let sm: String = iter.next().unwrap().parse().expect("Invalid input");

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