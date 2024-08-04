use std::cmp::{max, min};
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read total test cases
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap(); // Read n, k, a, b
        let values: Vec<usize> = input.trim().split_whitespace()
            .map(|s| s.parse().unwrap()).collect();

        let (n, k, mut a, mut b) = (values[0], values[1], values[2], values[3]);
        
        let m = min(n, k);
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap(); // Read array arr
        for (i, value) in input.trim().split_whitespace().enumerate() {
            arr[i + 1] = value.parse().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap(); // Read array aa
        for (i, value) in input.trim().split_whitespace().enumerate() {
            aa[i + 1] = value.parse().unwrap();
        }

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = max(p1, aa[a] * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a]; // Update `a`
            p2 = max(p2, aa[b] * (k - x + 1) as i64 + p2_);
            p2_ += aa[b];
            b = arr[b]; // Update `b`
        }

        // Print result based on the comparisons of p1 and p2
        if p1 > p2 {
            println!("Bodya");
        } else if p1 < p2 {
            println!("Sasha");
        } else {
            println!("Draw");
        }
    }
}