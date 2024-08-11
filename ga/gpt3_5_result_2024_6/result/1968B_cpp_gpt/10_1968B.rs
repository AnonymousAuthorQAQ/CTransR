use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        
        let x: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();

        let mut a = String::new();
        io::stdin().read_line(&mut a).unwrap();

        let mut b = String::new();
        io::stdin().read_line(&mut b).unwrap();

        let a: Vec<char> = a.trim().chars().collect();
        let b: Vec<char> = b.trim().chars().collect();

        let mut ans = 0;
        for i in b.iter() {
            if ans < a.len() && a[ans] == *i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}