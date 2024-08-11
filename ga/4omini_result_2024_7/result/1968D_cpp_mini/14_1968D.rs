use std::io::{self, Read};

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
    let input = stdin.lock().bytes().map(|b| b.unwrap() as char).collect::<String>();
    let mut iter = input.split_whitespace();
    
    let t: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: i64 = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based index
        let ps2: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based index
        
        let mut p: Vec<usize> = Vec::with_capacity(n);
        for _ in 0..n {
            let value: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based index
            p.push(value);
        }

        let mut a: Vec<i64> = Vec::with_capacity(n);
        for _ in 0..n {
            let value: i64 = iter.next().unwrap().parse().unwrap();
            a.push(value);
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