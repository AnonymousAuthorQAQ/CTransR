use std::cmp::{min, max};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|x| x.parse().unwrap());
        let n: usize = nums.next().unwrap();
        let k: usize = nums.next().unwrap();
        let a: usize = nums.next().unwrap();
        let b: usize = nums.next().unwrap();
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 0..m {
            p1 = max(p1, aa[a] as i64 * (k - x) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a];
            p2 = max(p2, aa[b] as i64 * (k - x) as i64 + p2_);
            p2_ += aa[b] as i64;
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