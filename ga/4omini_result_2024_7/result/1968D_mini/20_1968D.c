use std::cmp;
use std::io;

const N: usize = 200000;

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut a: usize = iter.next().unwrap().parse().unwrap();
        let mut b: usize = iter.next().unwrap().parse().unwrap();
        let m = cmp::min(n, k);
        
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let iter = input.split_whitespace();
        for (i, value) in iter.enumerate() {
            arr[i + 1] = value.parse::<usize>().unwrap();
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let iter = input.split_whitespace();
        for (i, value) in iter.enumerate() {
            aa[i + 1] = value.parse::<i64>().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        
        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k as i64 - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];
            
            p2 = maxl(p2, aa[b] * (k as i64 - x as i64 + 1) + p2_);
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