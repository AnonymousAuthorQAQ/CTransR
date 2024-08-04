use std::cmp::{max, min};
use std::io::{self, Read};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_to_string(&mut input).unwrap();
    
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut a: usize = iter.next().unwrap().parse().unwrap();
        let mut b: usize = iter.next().unwrap().parse().unwrap();
        let m = min(n, k);
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        for x in 1..=n {
            arr[x] = iter.next().unwrap().parse::<i32>().unwrap();
        }
        for x in 1..=n {
            aa[x] = iter.next().unwrap().parse::<i32>().unwrap();
        }

        for x in 1..=m {
            p1 = max(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a] as usize;

            p2 = max(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
            p2_ += aa[b] as i64;
            b = arr[b] as usize;
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