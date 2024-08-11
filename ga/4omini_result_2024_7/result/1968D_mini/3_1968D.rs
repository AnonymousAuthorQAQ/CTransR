use std::cmp::{max, min};
use std::io::{self, BufRead};

const N: usize = 200_000;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut lines = reader.lines();
    
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
        
        let n = nums.next().unwrap();
        let k = nums.next().unwrap();
        let mut a = nums.next().unwrap();
        let mut b = nums.next().unwrap();
        
        let m = min(n, k);

        let mut arr = vec![0; n as usize + 1];
        for x in 1..=n {
            let line = lines.next().unwrap().unwrap();
            arr[x as usize] = line.parse::<i32>().unwrap();
        }

        let mut aa = vec![0; n as usize + 1];
        for x in 1..=n {
            let line = lines.next().unwrap().unwrap();
            aa[x as usize] = line.parse::<i32>().unwrap();
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 1..=m {
            p1 = max(p1, (aa[a as usize] as i64) * (k - x + 1) as i64 + p1_);
            p1_ += aa[a as usize] as i64;
            a = arr[a as usize];

            p2 = max(p2, (aa[b as usize] as i64) * (k - x + 1) as i64 + p2_);
            p2_ += aa[b as usize] as i64;
            b = arr[b as usize];
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