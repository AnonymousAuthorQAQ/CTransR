use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        let mut line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let k = parts.next().unwrap().parse::<i32>().unwrap();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();
        let m = n.min(k);

        let mut arr = [0; 200001];
        let mut aa = [0; 200001];

        for x in 1..=n {
            arr[x] = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=n {
            aa[x] = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=m {
            p1 = p1.max(1ll * aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = p2.max(1ll * aa[b] * (k - x + 1) + p2_);
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