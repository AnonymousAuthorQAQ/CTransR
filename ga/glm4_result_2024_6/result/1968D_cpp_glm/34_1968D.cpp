use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> u64 {
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; p.len()];
    let mut current_pos = pos;

    while !vis[current_pos] && k > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur + (k as u64) * (a[current_pos] as u64));
        cur += a[current_pos];
        current_pos = p[current_pos];
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize;
        let k: usize;
        let ps1: usize;
        let ps2: usize;
        let mut p: Vec<usize> = Vec::new();
        let mut a: Vec<usize> = Vec::new();

        {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            n = parts[0];
            k = parts[1];
            ps1 = parts[2];
            ps2 = parts[3];
        }

        for _ in 0..n {
            let line = lines.next().unwrap().unwrap();
            let parts: Vec<usize> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            p.push(parts[0]);
            a.push(parts[1]);
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