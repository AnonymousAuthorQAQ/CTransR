use std::io::{self, BufRead};

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let mut a: usize = iter.next().unwrap().parse().unwrap();
        let mut b: usize = iter.next().unwrap().parse().unwrap();
        
        let m = min(n, k);
        
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        
        // Read arr
        let arr_line = lines.next().unwrap().unwrap();
        for (i, value) in arr_line.split_whitespace().enumerate() {
            arr[i + 1] = value.parse().unwrap();
        }
        
        // Read aa
        let aa_line = lines.next().unwrap().unwrap();
        for (i, value) in aa_line.split_whitespace().enumerate() {
            aa[i + 1] = value.parse().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64 * (k - x + 1) as i64) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, (aa[b] as i64 * (k - x + 1) as i64) + p2_);
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