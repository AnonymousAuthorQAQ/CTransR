use std::io;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let parts: Vec<usize> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = parts[0];
        let k = parts[1];
        let mut a = parts[2];
        let mut b = parts[3];
        
        let m = min(n, k);
        
        let mut arr: Vec<usize> = vec![0; n + 1];
        let mut aa: Vec<i64> = vec![0; n + 1];
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            arr[i + 1] = val.parse().unwrap();
        }
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            aa[i + 1] = val.parse().unwrap();
        }
        
        let (mut p1, mut p2) = (0, 0);
        let (mut p1_, mut p2_) = (0, 0);
        
        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];
            
            p2 = maxl(p2, aa[b] * (k - x + 1) + p2_);
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