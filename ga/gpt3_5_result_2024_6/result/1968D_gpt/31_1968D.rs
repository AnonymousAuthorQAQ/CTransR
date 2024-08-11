use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        let n = nums[0] as usize;
        let k = nums[1] as usize;
        let mut a = nums[2] as usize;
        let mut b = nums[3] as usize;
        let m = min(n, k);
        let mut a_ = a;
        let mut b_ = b;

        let mut arr: Vec<i32> = Vec::with_capacity(n + 1);
        let mut aa: Vec<i32> = Vec::with_capacity(n + 1);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        arr.extend(input.split_whitespace().map(|x| x.parse().unwrap()));

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        aa.extend(input.split_whitespace().map(|x| x.parse().unwrap()));

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        for x in 1..=m {
            p1 = max(p1, (aa[a - 1] * (k - x as i32 + 1) as i32 + p1_));
            p1_ += aa[a - 1];
            a = arr[a - 1] as usize;

            p2 = max(p2, (aa[b - 1] * (k - x as i32 + 1) as i32 + p2_));
            p2_ += aa[b - 1];
            b = arr[b - 1] as usize;
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