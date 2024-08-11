use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    while t > 0 {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut j = -1;

        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n = iter.next().unwrap().parse::<i32>().unwrap();
        let k = iter.next().unwrap().parse::<i32>().unwrap();
        let a = iter.next().unwrap().parse::<i32>().unwrap();
        let b = iter.next().unwrap().parse::<i32>().unwrap();

        let mut arr = vec![0; n as usize + 1];
        let mut aa = vec![0; n as usize + 1];

        for x in 1..=n {
            arr[x] = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=n {
            aa[x] = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=k.min(n) {
            p1 = p1.max(p1_ + aa[a] * (k - x + 1));
            p1_ += aa[a];
            a = arr[a];

            p2 = p2.max(p2_ + aa[b] * (k - x + 1));
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });

        t -= 1;
    }
}