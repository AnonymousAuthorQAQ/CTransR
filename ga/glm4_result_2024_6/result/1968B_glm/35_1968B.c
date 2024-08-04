use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim().parse::<i32>().expect("Please type a number!");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let n = parts.next().expect("Please type n!").parse::<i32>().expect("Please type a number!");
        let m = parts.next().expect("Please type m!").parse::<i32>().expect("Please type a number!");
        let sn = parts.next().expect("Please type sn!").to_string();
        let sm = parts.next().expect("Please type sm!").to_string();

        let mut i = 0;
        let mut j = 0;

        while i < n as usize && j < m as usize {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}