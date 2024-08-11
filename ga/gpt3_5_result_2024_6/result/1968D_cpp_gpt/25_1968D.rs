use std::io::{self, Write};
use std::vec::Vec;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64 * a[pos] as i64));
        cur += a[pos] as i64;
        pos = p[pos];
        k -= 1;
    }
    return mx;
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();

    for _ in 0..t {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs: Vec<usize> = input_line.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();
        let n = inputs[0];
        let mut k = inputs[1];
        let ps1 = inputs[2];
        let ps2 = inputs[3];

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let p: Vec<usize> = input_line.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let a: Vec<usize> = input_line.split_whitespace().map(|x| x.trim().parse().unwrap()).collect();

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