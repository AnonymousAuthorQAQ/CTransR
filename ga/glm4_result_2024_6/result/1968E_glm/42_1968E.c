use std::io;

fn solve(n: i32) {
    let mut t = n;
    println!("1 1");
    println!("2 1");
    t -= 2;
    if t > 0 {
        println!("{} {}", n, n);
        t -= 1;
    }
    for i in 3..3 + t {
        println!("1 {}", i);
    }
}

fn main() {
    let mut t = 1;
    io::stdin().read_line(&mut t).unwrap();
    for _ in 0..t {
        let mut n = String::new();
        io::stdin().read_line(&mut n).unwrap();
        solve(n.trim().parse::<i32>().unwrap());
    }
}