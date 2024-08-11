use std::io;

const N: usize = 200000;

fn min(a: usize, b: usize) -> usize {
    a.min(b)
}

fn maxl(a: u64, b: u64) -> u64 {
    a.max(b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut values = input.trim().split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let k: usize = values.next().unwrap().parse().unwrap();
        let a: usize = values.next().unwrap().parse().unwrap();
        let b: usize = values.next().unwrap().parse().unwrap();

        let mut arr = vec![0; N + 1];
        let mut aa = vec![0; N + 1];

        for i in 1..=n {
            arr[i] = i;
        }

        for i in 1..=n {
            aa[i] = i;
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;
        let mut a = a;
        let mut b = b;

        for _ in 1..=k {
            p1 = maxl(p1, 1u64 * aa[a] as u64 * (k as u64));
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, 1u64 * aa[b] as u64 * (k as u64));
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