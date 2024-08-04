use std::cmp::min;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();

        let mut arr: Vec<usize> = Vec::with_capacity(n + 1);
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        arr.extend(input.split_whitespace().map(|x| x.parse().unwrap()));

        let mut aa: Vec<usize> = Vec::with_capacity(n + 1);
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        aa.extend(input.split_whitespace().map(|x| x.parse().unwrap()));

        let m = min(n, k);
        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);

        let (mut a_, mut b_) = (a, b);
        for x in 1..=m {
            p1 = p1.max(aa[a - 1] * (k - x + 1) as i64 + p1_);
            p1_ += aa[a - 1];
            a = arr[a - 1];

            p2 = p2.max(aa[b - 1] * (k - x + 1) as i64 + p2_);
            p2_ += aa[b - 1];
            b = arr[b - 1];
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