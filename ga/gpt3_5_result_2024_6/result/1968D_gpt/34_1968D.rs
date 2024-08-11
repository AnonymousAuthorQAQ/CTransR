use std::cmp;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        let m = cmp::min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        
        let mut arr: Vec<usize> = Vec::with_capacity(n + 1);
        let mut aa: Vec<usize> = Vec::with_capacity(n + 1);
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for _ in 0..n {
            arr.push(iter.next().unwrap().parse().unwrap());
        }
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for _ in 0..n {
            aa.push(iter.next().unwrap().parse().unwrap());
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        for x in 1..=m {
            p1 = cmp::max(p1, 1u64 * aa[a] * (k - x + 1) as u64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = cmp::max(p2, 1u64 * aa[b] * (k - x + 1) as u64 + p2_);
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