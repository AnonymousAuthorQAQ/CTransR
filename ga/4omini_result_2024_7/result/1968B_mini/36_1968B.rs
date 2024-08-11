use std::io;

fn main() {
    let mut input = String::new();
    
    // Reading number of test cases
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        // Reading n and m
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let m: usize = parts.next().unwrap().parse().unwrap();
        
        // Reading strings sn and sm
        let mut strings = String::new();
        io::stdin().read_line(&mut strings).unwrap();
        let sn = &strings[..n];
        let sm = &strings[n..];

        let mut i = 0;
        let mut j = 0;

        // Loop to find the length of the longest matching subsequence
        while i < n && j < m {
            if sn.as_bytes()[i] == sm.as_bytes()[j] {
                i += 1;
            }
            j += 1;              
        }

        // Output the result
        println!("{}", i);
    }
}