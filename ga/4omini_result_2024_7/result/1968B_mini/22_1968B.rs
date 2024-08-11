use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut lines = handle.lines();
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let nm_line = lines.next().unwrap().unwrap();
        let mut nm_iter = nm_line.split_whitespace();
        let n: usize = nm_iter.next().unwrap().parse().unwrap();
        let m: usize = nm_iter.next().unwrap().parse().unwrap();

        let sn = lines.next().unwrap().unwrap();
        let sm = lines.next().unwrap().unwrap();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}