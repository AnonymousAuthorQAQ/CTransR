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
        let values: Vec<usize> = input.split_whitespace()
                                       .map(|x| x.parse().unwrap())
                                       .collect();
        let n = values[0];
        let k = values[1];
        let mut a = values[2];
        let mut b = values[3];
        let m = min(n, k);
        
        let mut arr: Vec<usize> = vec![0; n + 1];
        let mut aa: Vec<i64> = vec![0; n + 1];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            aa[i + 1] = value.parse::<i64>().unwrap(); // Read weights
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            arr[i + 1] = value.parse::<usize>().unwrap(); // Read connections
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k - x as i64 + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = maxl(p2, aa[b] * (k - x as i64 + 1) + p2_);
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