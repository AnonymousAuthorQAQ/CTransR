use std::io;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let params: Vec<usize> = input.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
        let (n, k, a, b) = (params[0], params[1], params[2], params[3]);
        let m = min(n, k);
        
        let mut arr = vec![0; n + 1];
        let mut aa = vec![0; n + 1];

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            arr[i + 1] = value.parse().unwrap();
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().enumerate() {
            aa[i + 1] = value.parse().unwrap();
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        let mut a_index = a;
        let mut b_index = b;

        for x in 1..=m {
            p1 = maxl(p1, (aa[a_index] * (k - x + 1) as i64) + p1_);
            p1_ += aa[a_index];
            a_index = arr[a_index];

            p2 = maxl(p2, (aa[b_index] * (k - x + 1) as i64) + p2_);
            p2_ += aa[b_index];
            b_index = arr[b_index];
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