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
    let input = stdin.lock();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let n = nums[0];
        let k = nums[1];
        let mut a = nums[2] - 1; // adapting to 0-indexed
        let mut b = nums[3] - 1; // adapting to 0-indexed

        let m = min(n, k);
        let mut arr: Vec<usize> = vec![0; n];
        let mut aa: Vec<i64> = vec![0; n];

        let line = lines.next().unwrap().unwrap();
        arr.extend(line.split_whitespace().map(|s| s.parse::<usize>().unwrap() - 1)); // adapting to 0-indexed
        
        let line = lines.next().unwrap().unwrap();
        aa.extend(line.split_whitespace().map(|s| s.parse::<i64>().unwrap()));

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 0..m {
            p1 = maxl(p1, aa[a] * (k as i64 - x as i64) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = maxl(p2, aa[b] * (k as i64 - x as i64) + p2_);
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