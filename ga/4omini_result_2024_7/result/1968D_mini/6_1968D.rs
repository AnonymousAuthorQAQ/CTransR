use std::io::{self, BufRead};

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut inputs = line.split_whitespace();
        
        let n: usize = inputs.next().unwrap().parse().unwrap();
        let k: usize = inputs.next().unwrap().parse().unwrap();
        let mut a: usize = inputs.next().unwrap().parse().unwrap() - 1; // Convert to 0-indexed
        let mut b: usize = inputs.next().unwrap().parse().unwrap() - 1; // Convert to 0-indexed
        
        let m = min(n, k);
        
        let mut arr = vec![0; n];
        let mut aa = vec![0; n];
        
        // Read first array
        let line = lines.next().unwrap().unwrap();
        arr.extend(line.split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1)); // Convert to 0-indexed
        
        // Read second array
        let line = lines.next().unwrap().unwrap();
        aa.extend(line.split_whitespace().map(|x| x.parse::<i64>().unwrap()));

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 0..m {
            p1 = maxl(p1, aa[a] * (k - x as i64) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, aa[b] * (k - x as i64) + p2_);
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