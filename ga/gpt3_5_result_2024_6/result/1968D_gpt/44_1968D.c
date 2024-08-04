use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace().map(|x| x.parse::<i64>().unwrap());
        
        let n = values.next().unwrap();
        let k = values.next().unwrap();
        let mut a = values.next().unwrap();
        let mut b = values.next().unwrap();
        let m = min(n, k as usize);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i64> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut p1 = 0;
        let mut p1_ = 0;
        let mut p2 = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = max(p1, aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize] as i64;
            p2 = max(p2, aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize] as i64;
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