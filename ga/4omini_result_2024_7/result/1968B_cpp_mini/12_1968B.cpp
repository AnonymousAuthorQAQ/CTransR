use std::io;

fn main() {
    let stdin = io::stdin();
    let t = read_usize(&stdin);
    
    for _ in 0..t {
        let x_k: Vec<usize> = read_vec::<usize>(&stdin, 2);
        let x = x_k[0]; // Ignored, as it is not used in the logic
        let k = x_k[1]; // Ignored, as it is not used in the logic
        
        let a: String = read_line(&stdin);
        let b: String = read_line(&stdin);

        let mut ans = 0;
        for char_b in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] == char_b as u8 {
                ans += 1;
            }
        }
        
        println!("{}", ans);
    }
}

fn read_usize(stdin: &io::Stdin) -> usize {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().parse().expect("Failed to parse")
}

fn read_line(stdin: &io::Stdin) -> String {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer.trim().to_string()
}

fn read_vec<T: std::str::FromStr>(stdin: &io::Stdin, n: usize) -> Vec<T> {
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).expect("Failed to read line");
    buffer
        .trim()
        .split_whitespace()
        .take(n)
        .map(|s| s.parse::<T>().expect("Failed to parse"))
        .collect()
}