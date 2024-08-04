use std::io::{self, BufRead};

const N: usize = 200000;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();
    
    input.read_line(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let mut values = buffer.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        
        let n = values.next().unwrap() as usize;
        let k = values.next().unwrap() as usize;
        let a = values.next().unwrap() as usize;
        let b = values.next().unwrap() as usize;
        let m = usize::min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let arr: Vec<i64> = buffer.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let aa: Vec<i64> = buffer.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        
        for x in 1..=m {
            p1 = p1.max(aa[a - 1] * (k - x + 1) as i64 + p1_);
            p1_ += aa[a - 1];
            a = arr[a - 1] as usize;
            
            p2 = p2.max(aa[b - 1] * (k - x + 1) as i64 + p2_);
            p2_ += aa[b - 1];
            b = arr[b - 1] as usize;
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