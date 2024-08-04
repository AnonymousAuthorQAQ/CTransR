use std::io;

fn calculate(p: &[usize], a: &[i32], pos: usize, k: usize) -> i64 {
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; p.len()];
    let mut pos = pos;

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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let k: usize = parts.next().unwrap().parse().unwrap();
        let ps1: usize = parts.next().unwrap().parse().unwrap();
        let ps2: usize = parts.next().unwrap().parse().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for part in input.trim().split_whitespace() {
            p.push(part.parse().unwrap());
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        for part in input.trim().split_whitespace() {
            a.push(part.parse().unwrap());
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