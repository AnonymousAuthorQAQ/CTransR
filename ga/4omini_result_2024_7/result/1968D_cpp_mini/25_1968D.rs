use std::io;

fn calculate(p: &[usize], a: &[i64], mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + k * a[pos]);
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: i64 = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse().unwrap();
        let ps2: usize = iter.next().unwrap().parse().unwrap();

        let mut p = vec![0; n];
        let mut a = vec![0; n];

        for i in 0..n {
            input.clear();
            stdin.read_line(&mut input).expect("Failed to read line");
            p[i] = input.trim().parse::<usize>().unwrap() - 1; // adjust for 0-indexing
        }

        for i in 0..n {
            input.clear();
            stdin.read_line(&mut input).expect("Failed to read line");
            a[i] = input.trim().parse::<i64>().unwrap();
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k); // adjust for 0-indexing
        let ans2 = calculate(&p, &a, ps2 - 1, k); // adjust for 0-indexing

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}