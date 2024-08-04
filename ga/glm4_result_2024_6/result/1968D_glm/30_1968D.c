use std::io;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t = input.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n = parts.next().unwrap().parse::<i32>().unwrap();
        let k = parts.next().unwrap().parse::<i32>().unwrap();
        let a = parts.next().unwrap().parse::<i32>().unwrap();
        let b = parts.next().unwrap().parse::<i32>().unwrap();
        let m = min(n, k);

        let mut arr = [0; 200001];
        let mut aa = [0; 200001];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[i] = input.trim().parse::<i32>().unwrap();
        }

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa[i] = input.trim().parse::<i32>().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;

        for _ in 1..=m {
            p1 = p1.max(1ll * aa[a] * (k as i64) + p1);
            p1 += aa[a];
            a = arr[a];

            p2 = p2.max(1ll * aa[b] * (k as i64) + p2);
            p2 += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}