use std::io;
use std::cmp::min;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut nums = input.trim().split_whitespace();
        let n: i32 = nums.next().unwrap().parse().unwrap();
        let k: i32 = nums.next().unwrap().parse().unwrap();
        let a: i32 = nums.next().unwrap().parse().unwrap();
        let b: i32 = nums.next().unwrap().parse().unwrap();

        let mut arr = vec![0; (n + 1) as usize];
        let mut aa = vec![0; (n + 1) as usize];

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            arr[i as usize] = input.trim().parse().unwrap();
        }

        for i in 1..=n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            aa[i as usize] = input.trim().parse().unwrap();
        }

        let m = min(n, k);
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 1..=m {
            p1 = p1.max(p1_ + aa[a as usize]);
            p1_ += aa[a as usize];
            a = arr[a as usize];

            p2 = p2.max(p2_ + aa[b as usize]);
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