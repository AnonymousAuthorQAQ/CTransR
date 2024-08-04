use std::io;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().expect("Invalid input");
        let k: usize = parts.next().unwrap().parse().expect("Invalid input");
        let a: usize = parts.next().unwrap().parse().expect("Invalid input");
        let b: usize = parts.next().unwrap().parse().expect("Invalid input");

        let m = min(n as i32, k as i32);
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        for x in 1..=n {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            arr[x] = input.trim().parse().expect("Invalid input");
        }

        for x in 1..=n {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            aa[x] = input.trim().parse().expect("Invalid input");
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a_ = a;
        let mut b_ = b;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a_] as i64) * (k as i64 - x as i64 + 1) + p1_);
            p1_ += aa[a_];
            a_ = arr[a_];
            
            p2 = maxl(p2, (aa[b_] as i64) * (k as i64 - x as i64 + 1) + p2_);
            p2_ += aa[b_];
            b_ = arr[b_];
        }

        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}