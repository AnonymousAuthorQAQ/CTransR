use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Please enter a number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().expect("Please enter a number").parse().expect("Please enter a valid number");
        let m: usize = parts.next().expect("Please enter a number").parse().expect("Please enter a valid number");
        let sn = parts.next().expect("Please enter a string").chars().take(n).collect::<String>();
        let sm = parts.next().expect("Please enter a string").chars().take(m).collect::<String>();

        let mut i = 0;
        let mut j = 0;
        while i < sn.len() && j < sm.len() {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}