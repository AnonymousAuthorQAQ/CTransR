use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let t: usize = input.trim().parse().expect("Not a number");
    
    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        let n: usize = parts[0].parse().expect("Not a number");
        let m: usize = parts[1].parse().expect("Not a number");
        
        let sn = parts[2];
        let sm = parts[3];

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