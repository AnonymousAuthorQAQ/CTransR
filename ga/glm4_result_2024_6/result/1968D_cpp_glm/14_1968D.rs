use std::io;
use std::io::prelude::*;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = std::cmp::max(mx, cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let mut reader = io::BufReader::new(stdin.lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let mut tokens = line.trim().split_whitespace();
        let n: usize = tokens.next().unwrap().parse().unwrap();
        let k: usize = tokens.next().unwrap().parse().unwrap();
        let ps1: usize = tokens.next().unwrap().parse().unwrap();
        let ps2: usize = tokens.next().unwrap().parse().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        for _ in 0..n {
            p.push(0);
            a.push(0);
        }

        for i in 0..n {
            p[i] = tokens.next().unwrap().parse().unwrap() - 1;
            a[i] = tokens.next().unwrap().parse().unwrap();
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);

        if ans1 == ans2 {
            writeln!(writer, "Draw").unwrap();
        } else if ans1 > ans2 {
            writeln!(writer, "Bodya").unwrap();
        } else {
            writeln!(writer, "Sasha").unwrap();
        }
    }
}