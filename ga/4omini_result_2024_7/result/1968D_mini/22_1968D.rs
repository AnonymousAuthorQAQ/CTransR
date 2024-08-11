use std::io;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    const N: usize = 200000;

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut a: usize = iter.next().unwrap().parse().unwrap();
        let mut b: usize = iter.next().unwrap().parse().unwrap();
        let m = min(n, k);

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            arr[i + 1] = val.parse().unwrap();
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            aa[i + 1] = val.parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * (k as i64 - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, (aa[b] as i64) * (k as i64 - x as i64 + 1) + p2_);
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