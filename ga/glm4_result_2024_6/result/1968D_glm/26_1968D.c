use std::io::{self, Read};

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

fn maxl(a: i64, b: i64) -> i64 {
    a.max(b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<i32>().unwrap();
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = parts.next().unwrap();
        let k = parts.next().unwrap();
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();

        let mut arr = [0; 200001];
        let mut aa = [0; 200001];
        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }
        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse::<i32>().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;
        for _ in 1..=min(n, k) {
            p1 = maxl(p1, 1i64 * aa[a] * (k - _ + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = maxl(p2, 1i64 * aa[b] * (k - _ + 1) + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}