use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.split_whitespace()
                                     .map(|s| s.parse().unwrap())
                                     .collect();
        let n = nums[0];
        let m = nums[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sn = input.trim();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sm = input.trim();

        let (mut i, mut j) = (0, 0);

        while i < n && j < m {
            if sn.chars().nth(i).unwrap() == sm.chars().nth(j).unwrap() {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}