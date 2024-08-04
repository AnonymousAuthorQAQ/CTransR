use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input)
        .expect("Failed to read input");
    let t: i32 = input.trim().parse()
        .expect("Invalid input");
    
    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");
        let values: Vec<i32> = input.split_whitespace()
            .map(|x| x.parse().expect("Invalid input"))
            .collect();
        let x = values[0];
        let k = values[1];
        
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");
        let mut a = String::new();
        a.push_str(&input.trim());
        
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read input");
        let mut b = String::new();
        b.push_str(&input.trim());
        
        let mut ans = 0;
        for c in b.chars() {
            if ans < a.len() && a.as_bytes()[ans] as char == c {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}