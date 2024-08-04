use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let mut nums = lines.next().unwrap().unwrap().split_whitespace().map(|s| s.parse::<usize>().unwrap());
        let n = nums.next().unwrap();
        let k = nums.next().unwrap();
        let a = nums.next().unwrap();
        let b = nums.next().unwrap();

        let mut arr = Vec::with_capacity(n + 1);
        let mut aa = Vec::with_capacity(n + 1);

        for _ in 0..n {
            arr.push(lines.next().unwrap().unwrap().parse::<usize>().unwrap());
        }

        for _ in 0..n {
            aa.push(lines.next().unwrap().unwrap().parse::<usize>().unwrap());
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut a = a;
        let mut b = b;

        for _ in 0..k {
            p1 = p1.max(aa[a] * (k - 1));
            a = arr[a];
            p2 = p2.max(aa[b] * (k - 1));
            b = arr[b];
        }

        println!("{}", if p1 > p2 { "Bodya" } else if p1 < p2 { "Sasha" } else { "Draw" });
    }
}