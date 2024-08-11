use std::io::{self, Write};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut pos = pos;

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
    let stdout = io::stdout();
    let mut reader = io::BufReader::new(io::stdin());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: i64 = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse().unwrap();
        let ps2: usize = iter.next().unwrap().parse().unwrap();

        input.clear();
        reader.read_line(&mut input).unwrap();
        let p: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse().unwrap())
            .map(|x: usize| x - 1)
            .collect();

        input.clear();
        reader.read_line(&mut input).unwrap();
        let a: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}