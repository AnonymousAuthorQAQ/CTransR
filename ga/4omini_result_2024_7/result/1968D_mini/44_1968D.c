use std::io;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let nums: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (n, k, mut a, mut b) = (nums[0], nums[1], nums[2], nums[3]);
        let m = min(n, k);

        let mut arr = vec![0; n + 1]; // A vector to hold arr. Using 1-based indexing.
        let mut aa = vec![0; n + 1];  // A vector to hold aa. Using 1-based indexing.

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        arr.iter_mut().skip(1).zip(input.split_whitespace())
            .for_each(|(v, s)| *v = s.parse().unwrap());

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        aa.iter_mut().skip(1).zip(input.split_whitespace())
            .for_each(|(v, s)| *v = s.parse().unwrap());

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a] as i64) * ((k - x + 1) as i64) + p1_);
            p1_ += aa[a];
            a = arr[a];

            p2 = maxl(p2, (aa[b] as i64) * ((k - x + 1) as i64) + p2_);
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