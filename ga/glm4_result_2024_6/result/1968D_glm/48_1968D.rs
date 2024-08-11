use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut j = -1;
        let mut i_ = -1;
        let mut j_ = -1;

        let mut values = lines.next().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let a = values.next().unwrap();
        let b = values.next().unwrap();
        let m = std::cmp::min(n, k);
        let mut arr = vec![0; n as usize + 1];
        let mut aa = vec![0; n as usize + 1];

        for x in 1..=n {
            arr[x] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=n {
            aa[x] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=m {
            p1 = std::cmp::max(p1, 1i64 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = std::cmp::max(p2, 1i64 * aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize];
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