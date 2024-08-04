use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input.split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let n = parts[0];
        let m = parts[1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sn = input.trim();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sm = input.trim();

        let mut i = 0;
        let mut j = 0;
        let sn_chars: Vec<char> = sn.chars().collect();
        let sm_chars: Vec<char> = sm.chars().collect();

        while i < n && j < m {
            if sn_chars[i as usize] == sm_chars[j as usize] {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}