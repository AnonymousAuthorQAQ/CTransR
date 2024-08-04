use std::io::{self, BufRead};

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    const N: usize = 200000;
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        let n = nums.next().unwrap();
        let k = nums.next().unwrap();
        let mut a = nums.next().unwrap() as usize;
        let mut b = nums.next().unwrap() as usize;
        let m = min(n, k);
        
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        // Read arr
        let line = lines.next().unwrap().unwrap();
        for (i, num) in line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).enumerate() {
            arr[i + 1] = num;
        }
        
        // Read aa
        let line = lines.next().unwrap().unwrap();
        for (i, num) in line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).enumerate() {
            aa[i + 1] = num;
        }
        
        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        
        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * (k - x + 1) + p1_);
            p1_ += aa[a] as i64;
            a = arr[a] as usize;

            p2 = maxl(p2, (aa[b] as i64) * (k - x + 1) + p2_);
            p2_ += aa[b] as i64;
            b = arr[b] as usize;
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