use std::cmp::{max, min};

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let (n, k, mut a, mut b) = (values.next().unwrap(), values.next().unwrap(), values.next().unwrap(), values.next().unwrap());
        let m = min(n, k);
        let (mut p1, mut p2, mut p1_, mut p2_) = (0i64, 0i64, 0i64, 0i64);

        let mut arr: Vec<i32> = vec![0; N + 1];
        let mut aa: Vec<i32> = vec![0; N + 1];

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
        for x in 1..=n {
            arr[x] = values.next().unwrap();
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap());
        for x in 1..=n {
            aa[x] = values.next().unwrap();
        }

        for x in 1..=m {
            p1 = max(p1, 1i64 * aa[a as usize] * (k - x + 1) + p1_);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = max(p2, 1i64 * aa[b as usize] * (k - x + 1) + p2_);
            p2_ += aa[b as usize];
            b = arr[b as usize];
        }

        if p1 > p2 {
            println!("Bodya")
        } else if p1 < p2 {
            println!("Sasha")
        } else {
            println!("Draw")
        }
    }
}