use std::io;
use std::cmp::{min, max};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: i32 = input.trim().parse().expect("Invalid input");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut tokens = input.trim().split_whitespace();
        let n: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let k: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let a: i32 = tokens.next().unwrap().parse().expect("Invalid input");
        let b: i32 = tokens.next().unwrap().parse().expect("Invalid input");

        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            arr[i as usize] = input.trim().parse().expect("Invalid input");
        }

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            aa[i as usize] = input.trim().parse().expect("Invalid input");
        }

        let m = min(n, k);
        let mut a = a;
        let mut b = b;
        let mut p1 = 0;
        let mut p2 = 0;

        for _ in 0..m {
            p1 = max(p1, aa[a as usize] * (k - _ + 1) + p1);
            p1 += aa[a as usize];
            a = arr[a as usize];

            p2 = max(p2, aa[b as usize] * (k - _ + 1) + p2);
            p2 += aa[b as usize];
            b = arr[b as usize];
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