use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<i32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let x = input[0];
        let k = input[1];
        
        let symbol_a: Vec<char> = lines.next().unwrap().trim().chars().collect();
        let symbol_b: Vec<char> = lines.next().unwrap().trim().chars().collect();

        let mut ans = 0;
        for i in symbol_b {
            if ans < symbol_a.len() && symbol_a[ans] == i {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}