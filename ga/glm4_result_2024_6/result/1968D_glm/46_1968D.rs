use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        let mut tokens = input.trim().split_whitespace();
        let n = tokens.next().unwrap().parse::<i32>().unwrap();
        let k = tokens.next().unwrap().parse::<i32>().unwrap();
        let a = tokens.next().unwrap().parse::<i32>().unwrap();
        let b = tokens.next().unwrap().parse::<i32>().unwrap();
        let mut arr = vec![0; n as usize + 1];
        let mut aa = vec![0; n as usize + 1];

        for i in 1..=n {
            arr[i] = tokens.next().unwrap().parse::<i32>().unwrap();
        }
        for i in 1..=n {
            aa[i] = tokens.next().unwrap().parse::<i32>().unwrap();
        }

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 1..=k {
            p1 = std::cmp::max(p1, p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];

            p2 = std::cmp::max(p2, p2_ + aa[b]);
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