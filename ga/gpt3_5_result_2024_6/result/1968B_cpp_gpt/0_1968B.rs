use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let inputs: Vec<usize> = lines
            .next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let x = inputs[0];
        let k = inputs[1];

        let a: Vec<char> = lines.next().unwrap().chars().collect();
        let b: Vec<char> = lines.next().unwrap().chars().collect();

        let mut ans = 0;
        for i in b {
            if ans < a.len() && a[ans] == i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}