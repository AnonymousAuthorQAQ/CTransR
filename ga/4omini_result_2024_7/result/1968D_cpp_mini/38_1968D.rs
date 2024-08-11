use std::cmp::max;
use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    let mut pos = pos;
    let mut k = k;

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = max(mx, cur + (k as i64 * a[pos]));
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
    let t: u32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut params = input
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        let n = params.next().unwrap();
        let k = params.next().unwrap();
        let ps1 = params.next().unwrap() - 1;
        let ps2 = params.next().unwrap() - 1;

        let mut p: Vec<usize> = Vec::with_capacity(n);
        let mut a: Vec<i64> = Vec::with_capacity(n);

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for x in input.split_whitespace() {
            p.push(x.parse::<usize>().unwrap() - 1);
        }

        input.clear();
        stdin.read_line(&mut input).unwrap();
        for x in input.split_whitespace() {
            a.push(x.parse::<i64>().unwrap());
        }

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