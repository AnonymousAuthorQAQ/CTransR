use std::io;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
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
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut j = -1;
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let nums: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = nums[0];
        let k = nums[1];
        let mut a = nums[2] as usize;
        let mut b = nums[3] as usize;
        let m = min(n, k);
        
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, num) in input.trim().split_whitespace().enumerate() {
            arr[i + 1] = num.parse().unwrap();
        }
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, num) in input.trim().split_whitespace().enumerate() {
            aa[i + 1] = num.parse().unwrap();
        }
        
        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a] as usize;
            
            p2 = maxl(p2, (aa[b] as i64) * (k - x + 1) + p2_);
            p2_ += aa[b];
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