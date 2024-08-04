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
        let params: Vec<usize> = input.trim().split_whitespace()
                                        .map(|x| x.parse().unwrap())
                                        .collect();
        let n = params[0];
        let k = params[1];
        let mut a = params[2] - 1; // converting to 0-based index
        let mut b = params[3] - 1; // converting to 0-based index
        let m = min(n, k);

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let arr: Vec<usize> = input.trim().split_whitespace()
                                    .map(|x| x.parse().unwrap())
                                    .collect();
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let aa: Vec<i64> = input.trim().split_whitespace()
                                 .map(|x| x.parse().unwrap())
                                 .collect();

        let mut p1: i64 = 0;
        let mut p2: i64 = 0;
        let mut p1_: i64 = 0;
        let mut p2_: i64 = 0;

        for x in 0..m {
            p1 = maxl(p1, aa[a] * (k - x as i64) + p1_);
            p1_ += aa[a];
            a = arr[a] - 1; // converting to 0-based index

            p2 = maxl(p2, aa[b] * (k - x as i64) + p2_);
            p2_ += aa[b];
            b = arr[b] - 1; // converting to 0-based index
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