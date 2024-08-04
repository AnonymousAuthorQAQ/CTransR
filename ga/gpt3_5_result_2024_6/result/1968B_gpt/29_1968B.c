use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = iterator.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input: Vec<_> = iterator.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = input[0];
        let m = input[1];
        let sn = input[2].chars().collect::<Vec<char>>();
        let sm = input[3].chars().collect::<Vec<char>>();

        let mut i = 0;
        let mut j = 0;

        while i < n && j < m {
            if sn[i] == sm[j] {
                i += 1;
            }
            j += 1;
        }

        println!("{}", i);
    }
}