use std::cmp;

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
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
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).expect("error reading input");
    let t: i32 = t.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("error reading input");
        let mut iter = input.split_whitespace();
        
        let n: i32 = iter.next().unwrap().parse().expect("invalid input");
        let k: i32 = iter.next().unwrap().parse().expect("invalid input");
        let a: i32 = iter.next().unwrap().parse().expect("invalid input");
        let b: i32 = iter.next().unwrap().parse().expect("invalid input");

        let m = cmp::min(n as i32, k);
        let mut a_ = a;
        let mut b_ = b;
        let mut arr: Vec<i32> = vec![0; (N+1)];
        let mut aa: Vec<i32> = vec![0; (N+1)];
        
        input.clear();
        std::io::stdin().read_line(&mut input).expect("error reading input");
        let mut iter = input.split_whitespace();
        for x in 1..=(n as usize) {
            arr[x] = iter.next().unwrap().parse().expect("invalid input");
        }

        input.clear();
        std::io::stdin().read_line(&mut input).expect("error reading input");
        let mut iter = input.split_whitespace();
        for x in 1..=(n as usize) {
            aa[x] = iter.next().unwrap().parse().expect("invalid input");
        }

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;
        let mut i: i32 = -1;
        let mut i_: i32 = -1;
        let mut j: i32 = -1;
        let mut j_: i32 = -1;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a as usize] as i64) * (k - x + 1) as i64 + p1_);
            p1_ += aa[a as usize] as i64;
            a = arr[a as usize] as i32;
            p2 = maxl(p2, (aa[b as usize] as i64) * (k - x + 1) as i64 + p2_);
            p2_ += aa[b as usize] as i64;
            b = arr[b as usize] as i32;
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