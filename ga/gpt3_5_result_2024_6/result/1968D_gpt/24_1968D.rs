use std::cmp::{min, max};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace().map(|x| x.parse().unwrap());
        let (n, k, a, b) = (nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap(), nums.next().unwrap());
        let (mut p1, mut p2) = (0, 0);
        let (mut p1_, mut p2_) = (0, 0);
        let (mut i, mut i_, mut j, mut j_) = (-1, -1, -1, -1);
        let (mut a_, mut b_) = (a, b);
        let m = min(n, k);
        let (mut arr, mut aa): (Vec<i32>, Vec<i32>) = (vec![0; N + 1], vec![0; N + 1]);

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        arr[1..=n].iter_mut().for_each(|x| *x = input.trim().parse().unwrap());

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        aa[1..=n].iter_mut().for_each(|x| *x = input.trim().parse().unwrap());

        for x in 1..=m {
            p1 = max(p1, 1i64 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = max(p2, 1i64 * aa[b as usize] * (k - x + 1) + p2_);
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