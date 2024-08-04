use std::io::{self, BufRead};

fn calculate(p: &Vec<i32>, a: &Vec<i32>, pos: i32, k: i32) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut pos = pos as usize;
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n = iter.next().unwrap().parse::<i32>().unwrap();
        let k = iter.next().unwrap().parse::<i32>().unwrap();
        let ps1 = iter.next().unwrap().parse::<i32>().unwrap();
        let ps2 = iter.next().unwrap().parse::<i32>().unwrap();

        let mut p = Vec::with_capacity(n as usize);
        let mut a = Vec::with_capacity(n as usize);

        for _ in 0..n {
            p.push(iter.next().unwrap().parse::<i32>().unwrap() - 1);
        }

        for _ in 0..n {
            a.push(iter.next().unwrap().parse::<i32>().unwrap());
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