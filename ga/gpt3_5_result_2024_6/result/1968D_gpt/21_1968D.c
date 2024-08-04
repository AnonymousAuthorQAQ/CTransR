use std::io::{self, BufRead};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: isize = -1;
        let mut i_: isize = -1;
        let mut j: isize = -1;
        let mut j_: isize = -1;
        
        let input: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let (n, k, mut a, mut b) = (input[0], input[1], input[2], input[3]);
        
        let m = n.min(k);
        let (mut a_, mut b_) = (a, b);

        let arr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let aa: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

        for x in 0..m {
            p1 = p1.max((aa[a] as i64) * (k - x + 1) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a];
            p2 = p2.max((aa[b] as i64) * (k - x + 1) as i64 + p2_);
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