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
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        let first_line: Vec<usize> = lines.next().unwrap().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = first_line[0];
        let k = first_line[1] as i64;
        let ps1 = first_line[2] - 1;
        let ps2 = first_line[3] - 1;

        let mut p: Vec<usize> = Vec::with_capacity(n);
        let mut a: Vec<i64> = Vec::with_capacity(n);
        
        for i in 0..n {
            let val: usize = lines.next().unwrap().unwrap().parse().unwrap();
            p.push(val - 1);
        }

        for i in 0..n {
            let val: i64 = lines.next().unwrap().unwrap().parse().unwrap();
            a.push(val);
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