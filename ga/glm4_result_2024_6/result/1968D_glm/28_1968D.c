use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: i32 = lines.next().unwrap().parse().unwrap();

    while t > 0 {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut j = -1;
        let mut i_ = -1;
        let mut j_ = -1;

        let mut line = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let m = n.min(k);

        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];

        line = lines.next().unwrap();
        parts = line.split_whitespace();
        for x in 1..=n {
            arr[x as usize] = parts.next().unwrap().parse().unwrap();
        }

        line = lines.next().unwrap();
        parts = line.split_whitespace();
        for x in 1..=n {
            aa[x as usize] = parts.next().unwrap().parse().unwrap();
        }

        for x in 1..=m {
            p1 = p1.max(1ll * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            i = arr[i as usize];

            p2 = p2.max(1ll * aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
            j = arr[j as usize];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });

        t -= 1;
    }
}