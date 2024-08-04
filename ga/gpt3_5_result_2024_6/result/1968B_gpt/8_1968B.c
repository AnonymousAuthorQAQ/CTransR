use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(Result::unwrap);

    let t: i32 = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<i32> = iterator.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = input[0];
        let m = input[1];

        let sn: String = iterator.next().unwrap();
        let sm: String = iterator.next().unwrap();

        let sn_chars: Vec<char> = sn.chars().collect();
        let sm_chars: Vec<char> = sm.chars().collect();

        let (mut i, mut j) = (0, 0);
        while i < n && j < m {
            if sn_chars[i as usize] == sm_chars[j as usize] {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}