use std::collections::HashSet;
use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: usize) -> usize {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = HashSet::new();

    while !vis.contains(&pos) && k > 0 {
        vis.insert(pos);
        mx = std::cmp::max(mx, cur + (k as usize * a[pos] as usize));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let mut buffer = String::new();

    input.read_line(&mut buffer).unwrap();
    let t: usize = buffer.trim().parse().unwrap();

    for _ in 0..t {
        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let parts: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = parts[0];
        let k = parts[1];
        let ps1 = parts[2];
        let ps2 = parts[3];

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let p: Vec<usize> = buffer
            .split_whitespace()
            .map(|x| x.parse().unwrap() - 1)
            .collect();

        buffer.clear();
        input.read_line(&mut buffer).unwrap();
        let a: Vec<usize> = buffer.split_whitespace().map(|x| x.parse().unwrap()).collect();

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