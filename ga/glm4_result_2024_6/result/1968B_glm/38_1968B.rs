use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let n = values.next().unwrap().parse::<usize>().unwrap();
        let m = values.next().unwrap().parse::<usize>().unwrap();
        let sn = values.next().unwrap();
        let sm = values.next().unwrap();

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