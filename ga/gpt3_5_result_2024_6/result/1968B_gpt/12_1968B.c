use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut inputs = input.trim().split_whitespace();
        let n: usize = inputs.next().unwrap().parse().unwrap();
        let m: usize = inputs.next().unwrap().parse().unwrap();
        
        let mut sn = String::new();
        io::stdin().read_line(&mut sn).unwrap();
        let sn = sn.trim();
        
        let mut sm = String::new();
        io::stdin().read_line(&mut sm).unwrap();
        let sm = sm.trim();
        
        let (mut i, mut j) = (0, 0);
        
        while i < n && j < m {
            if sn.chars().nth(i) == sm.chars().nth(j) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}