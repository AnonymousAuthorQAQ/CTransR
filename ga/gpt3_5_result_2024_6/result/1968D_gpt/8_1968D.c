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
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    
    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let mut p1: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i64 = -1;
        let mut i_: i64 = -1;
        let mut j: i64 = -1;
        let mut j_: i64 = -1;
        
        let input: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input[0];
        let k = input[1];
        let a = input[2];
        let b = input[3];
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        
        let mut arr: Vec<usize> = Vec::with_capacity(N + 1);
        let mut aa: Vec<usize> = Vec::with_capacity(N + 1);
        
        let arr_input: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        arr.extend(&arr_input);
        
        let aa_input: Vec<usize> = lines.next().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        aa.extend(&aa_input);
        
        for x in 1..=m {
            p1 = maxl(p1, (aa[a-1] as i64) * ((k - x + 1) as i64) + p1_);
            p1_ += aa[a-1] as i64;
            a = arr[a-1];
            
            p2 = maxl(p2, (aa[b-1] as i64) * ((k - x + 1) as i64) + p2_);
            p2_ += aa[b-1] as i64;
            b = arr[b-1];
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