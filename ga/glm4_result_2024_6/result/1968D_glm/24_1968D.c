use std::io;
use std::cmp::min;
use std::cmp::max;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_prime = 0;
        let mut p2_prime = 0;
        let mut i = -1;
        let mut i_prime = -1;
        let mut j = -1;
        let mut j_prime = -1;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let v: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().unwrap()).collect();
        let n = v[0];
        let k = v[1];
        let a = v[2];
        let b = v[3];
        let m = min(n, k);
        let mut arr = [0; N + 1];
        let mut aa = [0; N + 1];
        
        for x in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[x] = input.trim().parse().unwrap();
        }
        
        for x in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa[x] = input.trim().parse().unwrap();
        }
        
        for x in 1..=m {
            p1 = max(p1, (aa[a] as i64) * (k - x + 1) as i64 + p1_prime);
            p1_prime += aa[a];
            a = arr[a];
            p2 = max(p2, (aa[b] as i64) * (k - x + 1) as i64 + p2_prime);
            p2_prime += aa[b];
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