use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let n: i32 = parts[0].parse().expect("Please enter a number");
        let m: i32 = parts[1].parse().expect("Please enter a number");
        let sn = parts[2];
        let sm = parts[3];

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