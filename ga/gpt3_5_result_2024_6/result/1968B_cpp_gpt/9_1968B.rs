use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a = a.trim();

        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();
        let b = b.trim();

        let mut ans = 0;

        for (i, c) in b.chars().enumerate() {
            if ans < a.len() && a.chars().nth(ans) == Some(c) {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}