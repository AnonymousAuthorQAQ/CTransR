use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i32 = -1;
        let mut i_: i32 = -1;
        let mut j: i32 = -1;
        let mut j_: i32 = -1;
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let mut a = values.next().unwrap();
        let mut b = values.next().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        for x in 1..=m {
            p1 = max(p1, 1i64 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];
            
            p2 = max(p2, 1i64 * aa[b as usize] * (k - x + 1) + p2_);
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