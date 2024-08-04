use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let inputs: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        
        let n = inputs[0];
        let m = inputs[1];

        let mut sn = String::new();
        io::stdin().read_line(&mut sn).unwrap();
        let sn = sn.trim();

        let mut sm = String::new();
        io::stdin().read_line(&mut sm).unwrap();
        let sm = sm.trim();

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