use std::io::{self, BufRead};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    
    let mut buffer = String::new();
    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut iter = buffer.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let m = n.min(k);
        let mut a_ = a;
        let mut b_ = b;

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let arr: Vec<usize> = buffer.split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let aa: Vec<usize> = buffer.split_whitespace()
                                   .map(|x| x.parse().unwrap())
                                   .collect();

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 0..m {
            p1 = p1.max(aa[a] * (k - x) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = p2.max(aa[b] * (k - x) + p2_);
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