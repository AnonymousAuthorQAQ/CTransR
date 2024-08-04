use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let inputs: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = inputs[0] as usize;
        let m = inputs[1] as usize;
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sn = input.trim();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let sm = input.trim();

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