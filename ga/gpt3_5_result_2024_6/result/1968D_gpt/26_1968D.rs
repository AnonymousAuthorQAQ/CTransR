use std::cmp;

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
        let m = cmp::min(n, k);
        let (mut a_, mut b_) = (a, b);

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let arr: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let (mut p1, mut p2, mut p1_, mut p2_) = (0, 0, 0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);

        for x in 1..=m {
            p1 = cmp::max(p1, aa[a] * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = cmp::max(p2, aa[b] * (k - x + 1) as i64 + p2_);
            p2_ += aa[b];
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}