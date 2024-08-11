use std::cmp;

const N: usize = 200000;

fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|x| x.parse().unwrap());
        let n: i32 = values.next().unwrap();
        let k: i32 = values.next().unwrap();
        let a: i32 = values.next().unwrap();
        let b: i32 = values.next().unwrap();
        let m = cmp::min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut a_idx: i32 = -1;
        let mut b_idx: i32 = -1;

        for x in 0..m {
            let idx = x as usize + 1;
            p1 = maxl(p1, (aa[a as usize] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];
            p2 = maxl(p2, (aa[b as usize] * (k - x + 1) as i64) + p2_);
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