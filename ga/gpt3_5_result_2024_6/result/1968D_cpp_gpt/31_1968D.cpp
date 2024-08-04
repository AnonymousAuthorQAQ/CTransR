use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

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
    let mut input_iter = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = input_iter.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let input_params: Vec<usize> = input_iter
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = input_params[0];
        let k = input_params[1];
        let ps1 = input_params[2] - 1;
        let ps2 = input_params[3] - 1;

        let p: Vec<usize> = input_iter
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();

        let a: Vec<usize> = input_iter
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let ans1 = calculate(&p, &a, ps1, k as i64);
        let ans2 = calculate(&p, &a, ps2, k as i64);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}