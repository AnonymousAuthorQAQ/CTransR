use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let t: usize = input.trim().parse().expect("Not a valid number");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let params: Vec<&str> = input.trim().split_whitespace().collect();
        
        let n: usize = params[0].parse().expect("Not a valid number");
        let m: usize = params[1].parse().expect("Not a valid number");
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let sn = input.trim().as_bytes();
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let sm = input.trim().as_bytes();
        
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