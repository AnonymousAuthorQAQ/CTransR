use std::io::{self, BufRead};

fn main() {
    // Use a buffered reader to read inputs
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        // Read x and k (though they are not used in this implementation)
        let xk_line = lines.next().unwrap().unwrap();
        let mut xk_iter = xk_line.split_whitespace();
        let _x: i32 = xk_iter.next().unwrap().parse().unwrap();
        let _k: i32 = xk_iter.next().unwrap().parse().unwrap();
        
        // Read strings a and b
        let a = lines.next().unwrap().unwrap();
        let b = lines.next().unwrap().unwrap();
        
        let mut ans = 0;
        
        // Iterate through each character in b to find matches in a
        for char_b in b.chars() {
            if ans < a.len() && a.chars().nth(ans) == Some(char_b) {
                ans += 1;
            }
        }
        
        // Output the result
        println!("{}", ans);
    }
}