use std::io::{self, BufRead};

const N: usize = 200_000;

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
        let params: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        
        let (n, k, mut a, mut b) = (params[0], params[1], params[2], params[3]);
        let m = min(n, k);
        
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        let arr_input: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        arr[1..=n].copy_from_slice(&arr_input);
        
        let aa_input: Vec<i32> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        aa[1..=n].copy_from_slice(&aa_input);
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        
        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * (k - x + 1) + p1_);
            p1_ += aa[a] as i64;
            a = arr[a] as usize;
            
            p2 = maxl(p2, (aa[b] as i64) * (k - x + 1) + p2_);
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