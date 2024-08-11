use std::cmp::{max, min};
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let (n, k, mut a, mut b) = (first_line[0], first_line[1], first_line[2], first_line[3]);
        let m = min(n, k);
        
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        
        let arr_line: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        arr[1..=n].copy_from_slice(&arr_line);

        let aa_line: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        aa[1..=n].copy_from_slice(&aa_line);

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = max(p1, aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];
            
            p2 = max(p2, aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize];
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