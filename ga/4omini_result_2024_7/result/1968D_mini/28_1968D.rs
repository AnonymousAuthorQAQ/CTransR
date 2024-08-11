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
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        
        let parts: Vec<i32> = input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = parts[0];
        let k = parts[1];
        let a = parts[2] - 1; // adjusting for zero-based index
        let b = parts[3] - 1; // adjusting for zero-based index
        
        let m = min(n, k) as usize;

        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        arr[..n as usize].copy_from_slice(&input.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>());
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        aa[..n as usize].copy_from_slice(&input.split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>());

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        let mut a_ = a;
        let mut b_ = b;

        for x in 0..m {
            p1 = maxl(p1, (aa[a_] as i64) * (k - x) as i64 + p1_);
            p1_ += aa[a_];
            a_ = arr[a_ as usize] - 1; // adjusting for zero-based index
            
            p2 = maxl(p2, (aa[b_] as i64) * (k - x) as i64 + p2_);
            p2_ += aa[b_];
            b_ = arr[b_ as usize] - 1; // adjusting for zero-based index
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