use std::io::{self, Write};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut pos = pos;
    
    let mut k = k;
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k * a[pos] as i64));
        cur += a[pos] as i64;
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let params: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = params[0];
        let k = params[1];
        let ps1 = params[2];
        let ps2 = params[3];

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let p: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: Vec<usize> = input.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

        let ans1 = calculate(&p, &a, ps1 - 1, k as i64);
        let ans2 = calculate(&p, &a, ps2 - 1, k as i64);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}