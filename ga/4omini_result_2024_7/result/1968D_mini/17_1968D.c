use std::cmp::{max, min};
use std::io::{self, Read};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    
    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let mut line = lines.next().unwrap().split_whitespace();
        let n: usize = line.next().unwrap().parse().unwrap();
        let k: usize = line.next().unwrap().parse().unwrap();
        let mut a: usize = line.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based
        let mut b: usize = line.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based
        let m = min(n, k);
        
        let mut arr: Vec<usize> = vec![0; n];
        let mut aa: Vec<i64> = vec![0; n];
        
        let arr_input: Vec<usize> = lines.next().unwrap().split_whitespace()
                                          .map(|x| x.parse().unwrap())
                                          .collect();
        arr.copy_from_slice(&arr_input);
        
        let aa_input: Vec<i64> = lines.next().unwrap().split_whitespace()
                                         .map(|x| x.parse().unwrap())
                                         .collect();
        aa.copy_from_slice(&aa_input);
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = max(p1, aa[a] * (k - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a] - 1;  // Convert to 0-based

            p2 = max(p2, aa[b] * (k - x as i64 + 1) + p2_);
            p2_ += aa[b];
            b = arr[b] - 1;  // Convert to 0-based
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