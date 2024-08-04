use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let m: i32 = iter.next().unwrap().parse().unwrap();
        let sn = iter.next().unwrap();
        let sm = iter.next().unwrap();
        
        let mut i = 0;
        let mut j = 0;
        while i < n && j < m {
            if sn.chars().nth(i as usize) == sm.chars().nth(j as usize) {
                i += 1;
            }
            j += 1;
        }
        
        println!("{}", i);
    }
}