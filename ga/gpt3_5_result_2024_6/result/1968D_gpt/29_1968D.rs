use std::cmp::min;
use std::cmp::max;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        
        let n: i32 = iter.next().unwrap().parse().unwrap();
        let k: i32 = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;
        let mut a_ = a;
        let mut b_ = b;
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        let m = min(n, k);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        
        for x in 1..=n {
            arr[x] = iter.next().unwrap().parse().unwrap();
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        for x in 1..=n {
            aa[x] = iter.next().unwrap().parse().unwrap();
        }

        for x in 1..=m {
            p1 = max(p1, (aa[a-1] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a-1];
            a = arr[a-1];

            p2 = max(p2, (aa[b-1] * (k - x + 1) as i64) + p2_);
            p2_ += aa[b-1];
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