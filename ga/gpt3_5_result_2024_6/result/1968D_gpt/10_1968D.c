use std::cmp;

const N: usize = 200000;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let k = parts[1];
        let a = parts[2];
        let b = parts[3];
        let m = cmp::min(n, k);
        let mut a_ = a;
        let mut b_ = b;
        let mut arr: Vec<i32> = vec![0; n + 1];
        let mut aa: Vec<i32> = vec![0; n + 1];

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut index = 1;
        for num in input.split_whitespace() {
            arr[index] = num.trim().parse().unwrap();
            index += 1;
        }

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut index = 1;
        for num in input.split_whitespace() {
            aa[index] = num.trim().parse().unwrap();
            index += 1;
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut i = -1;
        let mut i_ = -1;
        let mut j = -1;
        let mut j_ = -1;

        for x in 1..=m {
            p1 = cmp::max(p1, aa[a] as i64 * (k - x + 1) + p1_);
            p1_ += aa[a];
            a = arr[a];
            p2 = cmp::max(p2, aa[b] as i64 * (k - x + 1) + p2_);
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