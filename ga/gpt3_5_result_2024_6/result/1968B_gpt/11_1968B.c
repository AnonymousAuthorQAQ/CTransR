use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = iterator.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input_str = iterator.next().unwrap();
        let inputs: Vec<&str> = input_str.trim().split_whitespace().collect();
        let n: i32 = inputs[0].parse().unwrap();
        let m: i32 = inputs[1].parse().unwrap();
        let sn: String = inputs[2].to_string();
        let sm: String = inputs[3].to_string();

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