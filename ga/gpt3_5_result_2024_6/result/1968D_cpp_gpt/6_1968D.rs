use std::io::{self, Read};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k * a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();

    let t: usize = input.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let n: usize = input.next().unwrap().parse().unwrap();
        let k: i64 = input.next().unwrap().parse().unwrap();
        let ps1: usize = input.next().unwrap().parse().unwrap();
        let ps2: usize = input.next().unwrap().parse().unwrap();

        let mut p = vec![0; n];
        let mut a = vec![0; n];
        
        for i in 0..n {
            p[i] = input.next().unwrap().parse::<usize>().unwrap() - 1;
        }

        for i in 0..n {
            a[i] = input.next().unwrap().parse().unwrap();
        }

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