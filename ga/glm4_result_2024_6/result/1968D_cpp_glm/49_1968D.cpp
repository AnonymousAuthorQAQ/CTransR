use std::io;

fn calculate(p: &[usize], a: &[i32], pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
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
        let mut values = input.trim().split_whitespace();
        let n: usize = values.next().unwrap().parse().unwrap();
        let k: usize = values.next().unwrap().parse().unwrap();
        let ps1: usize = values.next().unwrap().parse().unwrap();
        let ps2: usize = values.next().unwrap().parse().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let mut values = input.trim().split_whitespace();
            p.push(values.next().unwrap().parse().unwrap());
            a.push(values.next().unwrap().parse().unwrap());
        }

        for i in 0..n {
            p[i] -= 1;
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