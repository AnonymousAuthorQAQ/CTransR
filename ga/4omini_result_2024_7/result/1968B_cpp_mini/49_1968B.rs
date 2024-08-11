use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap(); // Reading number of test cases
    
    for _ in 0..t {
        let params: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let x = params[0]; // Not used in the computation, similar to the original code
        let k = params[1]; // Not used in the computation, similar to the original code
        
        let a = lines.next().unwrap();
        let b = lines.next().unwrap();
        
        let mut ans = 0;
        
        for i in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == i as u8 {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}