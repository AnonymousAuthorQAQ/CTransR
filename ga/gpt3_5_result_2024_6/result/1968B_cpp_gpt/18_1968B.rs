use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(Result::unwrap);

    let t: usize = iterator.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let input: Vec<i32> = iterator.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let x = input[0] as usize;
        let k = input[1] as usize;

        let a: Vec<char> = iterator.next().unwrap().chars().collect();
        let b: Vec<char> = iterator.next().unwrap().chars().collect();
        
        let mut ans = 0;
        for i in 0..k {
            if i < x && a[i] == b[i] {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}