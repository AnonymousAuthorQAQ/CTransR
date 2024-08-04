use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim().parse::<i32>().expect("Failed to parse t");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.trim().split_whitespace();
        let x = parts.next().unwrap().parse::<i32>().expect("Failed to parse x");
        let k = parts.next().unwrap().parse::<i32>().expect("Failed to parse k");
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}