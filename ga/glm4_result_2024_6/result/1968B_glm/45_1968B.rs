use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    while t > 0 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let m = parts.next().unwrap().parse::<i32>().unwrap();
        let sn = parts.next().unwrap();
        let sm = parts.next().unwrap();

        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);

        t -= 1;
    }
}