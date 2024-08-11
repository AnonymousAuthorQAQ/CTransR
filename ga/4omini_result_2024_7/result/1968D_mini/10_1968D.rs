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
    
    // Read the number of test cases
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let mut a: usize = parts.next().unwrap().parse().unwrap();
        let mut b: usize = parts.next().unwrap().parse().unwrap();
        
        let m = min(n, k);
        
        let mut arr: Vec<usize> = vec![0; n + 1];
        let mut aa: Vec<i64> = vec![0; n + 1];
        
        // Read the first array (arr)
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.trim().split_whitespace().enumerate() {
            arr[i + 1] = value.parse().unwrap();
        }
        
        // Read the second array (aa)
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.trim().split_whitespace().enumerate() {
            aa[i + 1] = value.parse().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        
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