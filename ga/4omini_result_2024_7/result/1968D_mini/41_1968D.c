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

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut inputs = line.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        
        let n = inputs.next().unwrap() as usize;
        let k = inputs.next().unwrap() as usize;
        let mut a = inputs.next().unwrap() as usize;
        let mut b = inputs.next().unwrap() as usize;
        
        let m = min(n, k);
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];
        
        let line = lines.next().unwrap().unwrap();
        for (i, value) in line.split_whitespace().enumerate() {
            arr[i + 1] = value.parse::<i32>().unwrap() as usize;
        }
        
        let line = lines.next().unwrap().unwrap();
        for (i, value) in line.split_whitespace().enumerate() {
            aa[i + 1] = value.parse::<i32>().unwrap();
        }
        
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            
            p2 = maxl(p2, (aa[b] as i64) * (k - x + 1) as i64 + p2_);
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