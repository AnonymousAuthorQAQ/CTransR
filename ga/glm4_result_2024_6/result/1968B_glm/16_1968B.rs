use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim().parse::<i32>().expect("Failed to parse test cases");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let n = parts[0].parse::<usize>().expect("Failed to parse n");
        let m = parts[1].parse::<usize>().expect("Failed to parse m");
        let sn = parts[2];
        let sm = parts[3];

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