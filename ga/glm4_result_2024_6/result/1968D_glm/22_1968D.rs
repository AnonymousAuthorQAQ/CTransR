use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut parts = lines.next().unwrap().split_whitespace().map(|s| s.parse().unwrap());
        let n: usize = parts.next().unwrap();
        let k: usize = parts.next().unwrap();
        let a: usize = parts.next().unwrap();
        let b: usize = parts.next().unwrap();
        
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        
        for i in 1..=n {
            arr[i] = lines.next().unwrap().parse().unwrap();
        }
        
        for i in 1..=n {
            aa[i] = lines.next().unwrap().parse().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;
        
        for _ in 1..=k.min(n) {
            p1 = p1.max(p1_ + aa[a]);
            p1_ += aa[a];
            a = arr[a];
            
            p2 = p2.max(p2_ + aa[b]);
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