use std::io;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let params: Vec<usize> = input.split_whitespace()
                                       .map(|x| x.parse().unwrap())
                                       .collect();
        let (n, k, mut a, mut b) = (params[0], params[1], params[2], params[3]);

        let m = min(n, k);
        let mut arr = vec![0; n + 1]; // Using 1-indexed array
        let mut aa = vec![0; n + 1]; // Using 1-indexed array
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            arr[i + 1] = val.parse().unwrap();
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, val) in input.split_whitespace().enumerate() {
            aa[i + 1] = val.parse().unwrap();
        }

        let mut p1 = 0i64;
        let mut p2 = 0i64;
        let mut p1_ = 0i64;
        let mut p2_ = 0i64;

        for x in 1..=m {
            p1 = maxl(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a];
            
            p2 = maxl(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
            p2_ += aa[b] as i64;
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