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

    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let mut k: i64 = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse().unwrap();
        let ps2: usize = iter.next().unwrap().parse().unwrap();

        let mut p = vec![0; n];
        let mut a = vec![0; n];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for i in 0..n {
            p[i] = iter.next().unwrap().parse::<usize>().unwrap() - 1; // Decrement to zero index
        }
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        for i in 0..n {
            a[i] = iter.next().unwrap().parse::<i64>().unwrap();
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k); // ps1 is input as 1-indexed
        let ans2 = calculate(&p, &a, ps2 - 1, k); // ps2 is also input as 1-indexed
        
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}