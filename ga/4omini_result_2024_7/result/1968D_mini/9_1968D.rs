use std::io;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: i32 = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a: usize;
        let mut b: usize;

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut params = input.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let k: usize = params.next().unwrap().parse().unwrap();
        a = params.next().unwrap().parse::<usize>().unwrap();
        b = params.next().unwrap().parse::<usize>().unwrap();
        
        let m = min(n as i32, k as i32) as usize;
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            arr[i + 1] = value.parse().unwrap();
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            aa[i + 1] = value.parse().unwrap();
        }

        for x in 1..=m {
            p1 = maxl(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            
            p2 = maxl(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
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