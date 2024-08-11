use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let m: i32 = parts.next().unwrap().parse().unwrap();
        let sn = parts.next().unwrap();
        let sm = parts.next().unwrap();

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