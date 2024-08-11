use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let t: i32 = buffer.trim().parse().unwrap();
    
    while t > 0 {
        let mut parts = buffer.trim().split_whitespace();
        let n: i32 = parts.next().unwrap().parse().unwrap();
        let k: i32 = parts.next().unwrap().parse().unwrap();
        let a: i32 = parts.next().unwrap().parse().unwrap();
        let b: i32 = parts.next().unwrap().parse().unwrap();
        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];
        
        for x in 1..=n {
            arr[x as usize] = parts.next().unwrap().parse().unwrap();
        }
        
        for x in 1..=n {
            aa[x as usize] = parts.next().unwrap().parse().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;
        
        for _ in 1..=k {
            p1 = p1.max(1ll * aa[a as usize]);
            a = arr[a as usize];
        }
        
        for _ in 1..=k {
            p2 = p2.max(1ll * aa[b as usize]);
            b = arr[b as usize];
        }
        
        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
        
        t -= 1;
    }
}