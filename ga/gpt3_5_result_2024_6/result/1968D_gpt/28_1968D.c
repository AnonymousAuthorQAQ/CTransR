use std::io::{self, BufRead};

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize { if a < b { a } else { b } }
fn maxl(a: i64, b: i64) -> i64 { if a > b { a } else { b } }

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        let m = min(n, k);
        let mut arr: Vec<usize> = vec![0; n + 1];
        let mut aa: Vec<i64> = vec![0; n + 1];

        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        for x in 1..=n {
            arr[x] = iter.next().unwrap().parse().unwrap();
        }

        input_line.clear();
        input.read_line(&mut input_line).unwrap();
        let mut iter = input_line.split_whitespace();
        for x in 1..=n {
            aa[x] = iter.next().unwrap().parse().unwrap();
        }

        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);

        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = maxl(p2, aa[b] * (k - x + 1) as i64 + p2_);
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