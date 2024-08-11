use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k * a[pos]));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        let line = lines.next().unwrap().unwrap();
        let mut iter = line.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: i64 = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-indexed
        let ps2: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-indexed

        let mut p = vec![0; n];
        let mut a = vec![0; n];

        let line = lines.next().unwrap().unwrap();
        for (i, num) in line.split_whitespace().enumerate() {
            p[i] = num.parse::<usize>().unwrap() - 1; // Convert to 0-indexed
        }

        let line = lines.next().unwrap().unwrap();
        for (i, num) in line.split_whitespace().enumerate() {
            a[i] = num.parse::<i64>().unwrap(); // Use i64 to match C++ ll
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