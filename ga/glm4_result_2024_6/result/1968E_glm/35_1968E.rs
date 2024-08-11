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
    io::stdin().read_line(&mut t).expect("Failed to read line");
    let t: i32 = t.trim().parse().expect("Failed to parse input");

    while t > 0 {
        let mut n = 0;
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let n: i32 = n.trim().parse().expect("Failed to parse input");

        solve(n);
        t -= 1;
    }
}