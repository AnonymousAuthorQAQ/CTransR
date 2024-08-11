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

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut params = input.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let k: usize = params.next().unwrap().parse().unwrap();
        let mut a: usize = params.next().unwrap().parse().unwrap() - 1; // Convert to 0-based
        let mut b: usize = params.next().unwrap().parse().unwrap() - 1; // Convert to 0-based

        let m = min(n, k);
        let mut arr = vec![0; n];
        let mut aa = vec![0; n];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            arr[i] = val.parse().unwrap() - 1; // Convert to 0-based
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            aa[i] = val.parse().unwrap();
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 0..m {
            p1 = maxl(p1, (aa[a] as i64) * ((k - x) as i64 + 1) + p1_);
            p1_ += aa[a] as i64;
            a = arr[a];

            p2 = maxl(p2, (aa[b] as i64) * ((k - x) as i64 + 1) + p2_);
            p2_ += aa[b] as i64;
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