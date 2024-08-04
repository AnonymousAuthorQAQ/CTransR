use std::io;
use std::cmp::{min, max};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        let n = values.next().unwrap().parse::<i32>().unwrap();
        let k = values.next().unwrap().parse::<i32>().unwrap();
        let a = values.next().unwrap().parse::<i32>().unwrap();
        let b = values.next().unwrap().parse::<i32>().unwrap();

        let mut arr: Vec<i32> = (0..n+1).collect();
        let mut aa: Vec<i32> = (0..n+1).collect();

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        for x in 1..n+1 {
            arr[x] = values.next().unwrap().parse::<i32>().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace();
        for x in 1..n+1 {
            aa[x] = values.next().unwrap().parse::<i32>().unwrap();
        }

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for x in 1..=k {
            p1 = max(p1, 1LL * aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = max(p2, 1LL * aa[b] * (k - x + 1) + p2_);
            p2_ += aa[b];
            b = arr[b];
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