use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64 * a[pos]));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let (n, k, ps1, ps2) = (first_line[0], first_line[1], first_line[2] - 1, first_line[3] - 1);

        let p: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        let a: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

        let ans1 = calculate(&p, &a, ps1, k);
        let ans2 = calculate(&p, &a, ps2, k);

        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}