use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().parse().unwrap();
    while t > 0 {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let l: usize = line.next().unwrap().parse().unwrap();
        let r: usize = line.next().unwrap().parse().unwrap();
        let s = line.next().unwrap();

        let mut left = 0;
        let mut right = n + 1;

        while left + 1 < right {
            let mid = (left + right) / 2;
            if can_form_substrings(s.as_bytes(), mid, l) {
                left = mid;
            } else {
                right = mid;
            }
        }

        println!("{}", left);

        t -= 1;
    }
}

fn can_form_substrings(s: &[u8], len: usize, l: usize) -> bool {
    let mut count = 0;
    let mut i = 0;
    while i < s.len() {
        if s.len() - i < len {
            break;
        }
        if &s[i..i + len] == &s[i + len..i + 2 * len] {
            count += 1;
            i += len;
        } else {
            i += 1;
        }
    }
    count >= l - 1
}