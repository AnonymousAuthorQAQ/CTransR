use std::cmp::{min, max};
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    let mut lines = input.lines();
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().split_whitespace()
                                             .map(|x| x.parse().unwrap())
                                             .collect();
                                             
        let (n, k, a, b) = (first_line[0], first_line[1], first_line[2], first_line[3]);
        let m = min(n, k);
        
        let arr: Vec<usize> = lines.next().unwrap().split_whitespace()
                                     .map(|x| x.parse().unwrap())
                                     .collect();
                                     
        let aa: Vec<usize> = lines.next().unwrap().split_whitespace()
                                     .map(|x| x.parse().unwrap())
                                     .collect();
                                     
        let mut p1 = 0u64;
        let mut p2 = 0u64;
        let mut p1_ = 0u64;
        let mut p2_ = 0u64;
        
        let mut a_ind = a;
        let mut b_ind = b;
        
        for x in 0..m {
            p1 = max(p1, aa[a_ind] as u64 * (k - x + 1) + p1_);
            p1_ += aa[a_ind] as u64;
            a_ind = arr[a_ind];
            
            p2 = max(p2, aa[b_ind] as u64 * (k - x + 1) + p2_);
            p2_ += aa[b_ind] as u64;
            b_ind = arr[b_ind];
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