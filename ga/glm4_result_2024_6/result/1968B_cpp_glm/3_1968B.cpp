use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let k = iter.next().unwrap().parse::<i32>().unwrap();
        
        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }

        println!("{}", ans);
    }
}