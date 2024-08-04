use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _i in 0..t{
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut tk = input.trim().split_whitespace();
        let x: i32 = tk.next().unwrap().parse().unwrap();
        let k: i32 = tk.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let a = input.trim().to_string();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let b = input.trim().to_string();

        let mut ans = 0;
        for i in b.chars() {
            if ans < a.len() && a.chars().nth(ans).unwrap() == i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}