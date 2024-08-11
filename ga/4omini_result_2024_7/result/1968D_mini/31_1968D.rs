use std::cmp::{max, min};
use std::io::{self, Read};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        
        let (n, k, mut a, mut b) = (first_line[0], first_line[1], first_line[2], first_line[3]);
        let m = min(n, k);

        let arr: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let aa: Vec<usize> = lines.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;

        let mut a_ = a;
        let mut b_ = b;

        for x in 1..=m {
            p1 = max(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            
            p2 = max(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
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