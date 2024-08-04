use std::io;

fn min(a: usize, b: usize) -> usize {
    if a < b { a } else { b }
}

fn maxl(a: i64, b: i64) -> i64 {
    if a > b { a } else { b }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Invalid number");

    for _ in 0..t {
        input.clear();
        handle.read_line(&mut input).expect("Failed to read line");
        let params: Vec<usize> = input.split_whitespace()
            .map(|s| s.parse().expect("Invalid number"))
            .collect();
        
        let n = params[0];
        let k = params[1];
        let mut a = params[2] - 1; // Convert to 0-based index
        let mut b = params[3] - 1; // Convert to 0-based index
        let m = min(n, k);

        let mut arr = vec![0; n];
        input.clear();
        handle.read_line(&mut input).expect("Failed to read line");
        for (i, value) in input.split_whitespace().enumerate() {
            arr[i] = value.parse().expect("Invalid number") - 1; // Convert to 0-based index
        }

        let mut aa = vec![0; n];
        input.clear();
        handle.read_line(&mut input).expect("Failed to read line");
        for (i, value) in input.split_whitespace().enumerate() {
            aa[i] = value.parse().expect("Invalid number");
        }

        let mut p1 = 0;
        let mut p2 = 0;
        let mut p1_ = 0;
        let mut p2_ = 0;

        for x in 0..m {
            p1 = maxl(p1, (aa[a] as i64) * (k - x as i64) + p1_);
            p1_ += aa[a] as i64;
            a = arr[a]; // Move to the next index

            p2 = maxl(p2, (aa[b] as i64) * (k - x as i64) + p2_);
            p2_ += aa[b] as i64;
            b = arr[b]; // Move to the next index
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