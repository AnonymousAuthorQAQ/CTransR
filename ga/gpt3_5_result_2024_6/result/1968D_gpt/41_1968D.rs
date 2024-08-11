use std::cmp;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        let n: i32 = values.next().unwrap().parse().unwrap();
        let k: i32 = values.next().unwrap().parse().unwrap();
        let a: i32 = values.next().unwrap().parse().unwrap();
        let b: i32 = values.next().unwrap().parse().unwrap();

        let m = min(n, k);
        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        for x in 1..=n {
            arr[x] = values.next().unwrap().parse().unwrap();
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.split_whitespace();
        for x in 1..=n {
            aa[x] = values.next().unwrap().parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a_ = a;
        let mut b_ = b;

        for x in 1..=m {
            p1 = cmp::max(p1, (aa[a as usize] * (k - x + 1) + p1_));
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = cmp::max(p2, (aa[b as usize] * (k - x + 1) + p2_));
            p2_ += aa[b as usize];
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