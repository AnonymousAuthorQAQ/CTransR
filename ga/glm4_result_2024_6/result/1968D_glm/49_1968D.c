use std::io;
use std::cmp::min;
use std::cmp::max;

const N: usize = 200000;

fn main() {
    let mut t = 0;
    let mut n = 0;
    let mut k = 0;
    let mut a = 0;
    let mut b = 0;
    let mut arr = [0; N + 1];
    let mut aa = [0; N + 1];
    let mut p1 = 0;
    let mut p2 = 0;
    let mut p1_ = 0;
    let mut p2_ = 0;
    let mut i = -1;
    let mut i_ = -1;
    let mut j = -1;
    let mut j_ = -1;

    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    t = line.parse::<i32>().unwrap();

    while t > 0 {
        p1 = 0;
        p2 = 0;
        p1_ = 0;
        p2_ = 0;
        i = -1;
        i_ = -1;
        j = -1;
        j_ = -1;

        let line = stdin.lock().lines().next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        n = iter.next().unwrap().parse::<i32>().unwrap() as usize;
        k = iter.next().unwrap().parse::<i32>().unwrap() as usize;
        a = iter.next().unwrap().parse::<i32>().unwrap() as usize;
        b = iter.next().unwrap().parse::<i32>().unwrap() as usize;

        for x in 1..=n {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            arr[x] = line.parse::<i32>().unwrap();
        }

        for x in 1..=n {
            let line = stdin.lock().lines().next().unwrap().unwrap();
            aa[x] = line.parse::<i32>().unwrap();
        }

        for x in 1..=min(n, k) {
            p1 = max(p1, aa[a] as i64 * (k - x + 1) as i64 + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = max(p2, aa[b] as i64 * (k - x + 1) as i64 + p2_);
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

        t -= 1;
    }
}