use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap(); // read the number of test cases

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        let x: usize = parts[0].parse().unwrap(); // read x (unused)
        let k: usize = parts[1].parse().unwrap(); // read k (unused)
        
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a: String = input.trim().to_string(); // read string a

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b: String = input.trim().to_string(); // read string b

        // Calculate the number of matching characters
        let mut ans = 0;
        for ch in b.chars() {
            // Check if we're still matching within 'a'
            if ans < a.len() && a.chars().nth(ans).unwrap() == ch {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}