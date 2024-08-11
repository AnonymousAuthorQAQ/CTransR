use std::io;

const N: usize = 200_000;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut numbers = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    
    let t = numbers.next().unwrap();
    
    for _ in 0..t {
        let n = numbers.next().unwrap() as usize;
        let k = numbers.next().unwrap() as usize;
        let mut a = numbers.next().unwrap() as usize;
        let mut b = numbers.next().unwrap() as usize;

        let m = min(n, k);
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            arr[i] = input.trim().parse().unwrap();
        }

        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            aa[i] = input.trim().parse().unwrap();
        }

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
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