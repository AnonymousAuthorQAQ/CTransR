use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let inputs: Vec<i32> = input.split_whitespace().map(|x| x.parse().expect("Invalid input")).collect();
        let n = inputs[0];
        let m = inputs[1];
        
        let mut sn = String::new();
        io::stdin().read_line(&mut sn).expect("Failed to read input");
        let sn = sn.trim();
        
        let mut sm = String::new();
        io::stdin().read_line(&mut sm).expect("Failed to read input");
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