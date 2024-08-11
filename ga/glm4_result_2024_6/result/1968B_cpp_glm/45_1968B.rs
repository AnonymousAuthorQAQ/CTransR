use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t = input.lines().next().unwrap().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = input.lines().next().unwrap().split_whitespace();
        let x: i32 = line.next().unwrap().parse().unwrap();
        let k: i32 = line.next().unwrap().parse().unwrap();
        line.next(); // discard x
        line.next(); // discard k

        let a: String = line.next().unwrap().to_string();
        let b: String = line.next().unwrap().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}