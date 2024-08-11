use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t: i32 = input_line.trim().parse().unwrap();

    for _ in 0..t {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs: Vec<i32> = input_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let x = inputs[0];
        let k = inputs[1];

        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();
        let a = a.trim();

        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();
        let b = b.trim();

        let mut ans = 0;
        for c in b.chars() {
            if let Some(ch) = a.chars().nth(ans as usize) {
                if c == ch {
                    ans += 1;
                }
            }
        }
        println!("{}", ans);
    }
}