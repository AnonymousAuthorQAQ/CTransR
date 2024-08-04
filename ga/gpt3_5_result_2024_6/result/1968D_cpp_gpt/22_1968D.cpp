use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as u64 * a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = iterator.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let input: Vec<usize> = iterator.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = input[0];
        let k = input[1];
        let ps1 = input[2];
        let ps2 = input[3];

        let p: Vec<usize> = iterator.next().unwrap().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
        
        let a: Vec<usize> = iterator.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
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