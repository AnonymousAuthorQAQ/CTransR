use std::io;

const N: usize = 200000;

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
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let mut a: usize = parts.next().unwrap().parse().unwrap();
        let mut b: usize = parts.next().unwrap().parse().unwrap();
        
        let m = min(n, k);
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let arr_values: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let aa_values: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        for x in 0..n {
            arr[x + 1] = arr_values[x];
            aa[x + 1] = aa_values[x];
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, aa[a] * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, aa[b] * (k - x + 1) + p2_);
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