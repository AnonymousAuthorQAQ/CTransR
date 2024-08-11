use std::io;

const N: usize = 200_000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();
    
    stdin.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();
    
    for _ in 0..t {
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        let parts: Vec<usize> = buffer.trim().split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
        let n = parts[0];
        let k = parts[1];
        let mut a = parts[2] - 1;  // Convert to zero-based index
        let mut b = parts[3] - 1;  // Convert to zero-based index
        let m = min(n, k);
        
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        for (i, val) in buffer.trim().split_whitespace().map(|s| s.parse().unwrap()).enumerate() {
            arr[i + 1] = val;  // One-based index
        }

        buffer.clear();
        stdin.read_line(&mut buffer).unwrap();
        for (i, val) in buffer.trim().split_whitespace().map(|s| s.parse().unwrap()).enumerate() {
            aa[i + 1] = val;  // One-based index
        }
        
        let mut p1 = 0i64;
        let mut p2 = 0i64;
        let mut p1_ = 0i64;
        let mut p2_ = 0i64;
        
        for x in 1..=m {
            p1 = maxl(p1, aa[a + 1] * (k - x + 1) as i64 + p1_);
            p1_ += aa[a + 1] as i64;
            a = arr[a + 1] - 1;  // Update and convert back to zero-based index

            p2 = maxl(p2, aa[b + 1] * (k - x + 1) as i64 + p2_);
            p2_ += aa[b + 1] as i64;
            b = arr[b + 1] - 1;  // Update and convert back to zero-based index
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