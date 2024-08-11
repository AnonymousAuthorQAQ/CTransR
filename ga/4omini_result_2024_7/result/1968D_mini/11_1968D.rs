use std::io;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let params: Vec<usize> = input.split_whitespace()
                                       .map(|s| s.parse().unwrap())
                                       .collect();
        let n = params[0];
        let k = params[1];
        let mut a = params[2] - 1; // converting to 0-based index
        let mut b = params[3] - 1; // converting to 0-based index
        let m = min(n, k);

        let mut arr = vec![0; n];
        let mut aa = vec![0; n];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().map(|s| s.parse().unwrap()).enumerate() {
            arr[i] = value;
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for (i, value) in input.split_whitespace().map(|s| s.parse().unwrap()).enumerate() {
            aa[i] = value;
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 0..m {
            p1 = maxl(p1, aa[a] as i64 * (k - x) as i64 + p1_);
            p1_ += aa[a] as i64;
            a = arr[a] - 1; // update to 0-based index

            p2 = maxl(p2, aa[b] as i64 * (k - x) as i64 + p2_);
            p2_ += aa[b] as i64;
            b = arr[b] - 1; // update to 0-based index
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