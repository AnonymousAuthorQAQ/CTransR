use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = parts[0];
        let k = parts[1];
        let mut a = parts[2];
        let mut b = parts[3];
        let mut p1 = 0;
        let mut p2 = 0;
        let m = min(n, k);
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr_input: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        arr[1..].copy_from_slice(&arr_input);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa_input: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        aa[1..].copy_from_slice(&aa_input);

        for x in 1..=m {
            let mut p1_ = 0;
            let mut p2_ = 0;

            for _ in 0..(k - x + 1) {
                p1 = max(p1, aa[a] * (k - x + 1) + p1_);
                p1_ += aa[a];
                a = arr[a];
                
                p2 = max(p2, aa[b] * (k - x + 1) + p2_);
                p2_ += aa[b];
                b = arr[b];
            }
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