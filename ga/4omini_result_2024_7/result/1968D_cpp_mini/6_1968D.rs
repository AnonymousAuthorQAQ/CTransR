use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: i64) -> i64 {
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
    let handle = stdin.lock();
    let mut lines = handle.lines().map(|l| l.unwrap());

    let t: i32 = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let first_line: Vec<i32> = lines.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let n = first_line[0] as usize;
        let k = first_line[1];
        let ps1 = first_line[2] - 1; // Convert to 0-based index
        let ps2 = first_line[3] - 1; // Convert to 0-based index

        let mut p: Vec<usize> = lines.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1) // Convert to 0-based index
            .collect();

        let a: Vec<i64> = lines.next().unwrap()
            .trim()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let ans1 = calculate(&p, &a, ps1 as usize, k);
        let ans2 = calculate(&p, &a, ps2 as usize, k);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}