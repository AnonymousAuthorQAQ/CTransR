use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let values: Vec<i32> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = values[0];
        let m = values[1];
        
        let mut sn = String::new();
        io::stdin().read_line(&mut sn).unwrap();
        sn = sn.trim().to_string();
        
        let mut sm = String::new();
        io::stdin().read_line(&mut sm).unwrap();
        sm = sm.trim().to_string();
        
        let mut i = 0;
        let mut j = 0;
        
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}