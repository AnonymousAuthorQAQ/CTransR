use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();

    while t > 0 {
        let mut n = 0;
        let mut k = 0;
        let mut a = 0;
        let mut b = 0;
        let mut arr = [0; 200001];
        let mut aa = [0; 200001];

        let mut tokens = lines.next().unwrap().split_whitespace();
        n = tokens.next().unwrap().parse::<i32>().unwrap();
        k = tokens.next().unwrap().parse::<i32>().unwrap();
        a = tokens.next().unwrap().parse::<i32>().unwrap();
        b = tokens.next().unwrap().parse::<i32>().unwrap();

        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for _ in 1..=k {
            p1 = p1.max(p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];

            p2 = p2.max(p2_ + aa[b]);
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

        t -= 1;
    }
}