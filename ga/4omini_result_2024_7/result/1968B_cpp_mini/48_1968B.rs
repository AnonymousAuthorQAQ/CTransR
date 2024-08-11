use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        let mut a = String::new();
        let mut b = String::new();
        
        stdin.read_line(&mut a).unwrap();
        stdin.read_line(&mut b).unwrap();

        let mut ans = 0;
        let a_chars: Vec<_> = a.chars().collect();
        
        for i in b.chars() {
            if ans < a_chars.len() && a_chars[ans] == i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}