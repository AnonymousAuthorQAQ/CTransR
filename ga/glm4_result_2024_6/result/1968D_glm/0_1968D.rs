use std::io::{self, Read};

fn min(a: i32, b: i32) -> i32 {
    a.min(b)
}

fn maxl(a: i64, b: i64) -> i64 {
    a.max(b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");
    
    let mut lines = input.lines();
    let t: i32 = lines.next().unwrap().parse().expect("Invalid number of test cases");
    
    for _ in 0..t {
        let values: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let (n, k, a, b) = (values[0], values[1], values[2], values[3]);
        
        let arr: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let aa: Vec<i32> = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut a_ = a;
        let mut b_ = b;
        
        for _ in 0..k {
            p1 = maxl(p1, 1i64 * aa[a_] as i64 * (k as i64 - 1) + p1);
            p1 += aa[a_] as i64;
            a_ = arr[a_];
            
            p2 = maxl(p2, 1i64 * aa[b_] as i64 * (k as i64 - 1) + p2);
            p2 += aa[b_] as i64;
            b_ = arr[b_];
        }
        
        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}