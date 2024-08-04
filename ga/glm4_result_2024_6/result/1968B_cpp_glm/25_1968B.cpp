use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut nums = input.trim().split_whitespace();
        let x: usize = nums.next().unwrap().parse().expect("Invalid input");
        let k: usize = nums.next().unwrap().parse().expect("Invalid input");
        
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a: Vec<char> = input.trim().chars().collect();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let b: Vec<char> = input.trim().chars().collect();

        let mut ans = 0;
        for i in b {
            if ans < a.len() && a[ans] == i {
                ans += 1;
            }
        }
        println!("{}", ans);
    }
}