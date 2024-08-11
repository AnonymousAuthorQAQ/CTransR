use std::io;
use std::io::prelude::*;
use std::cmp::max;

fn calculate(p: &Vec<i32>, a: &Vec<i32>, pos: i32, k: i32) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut pos = pos as usize;

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = max(mx, cur as i64 + (k as i64 * a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }

    mx
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let t: i32 = read!(stdin, "{}");
    for _ in 0..t {
        let n: i32 = read!(stdin, "{}");
        let k: i32 = read!(stdin, "{}");
        let ps1: i32 = read!(stdin, "{}");
        let ps2: i32 = read!(stdin, "{}");

        let mut p = vec![0; n as usize];
        let mut a = vec![0; n as usize];

        for i in 0..n {
            p[i as usize] = read!(stdin, "{}") - 1;
        }

        for i in 0..n {
            a[i as usize] = read!(stdin, "{}");
        }

        let ans1 = calculate(&p, &a, ps1 - 1, k);
        let ans2 = calculate(&p, &a, ps2 - 1, k);

        if ans1 == ans2 {
            writeln!(stdout, "Draw").unwrap();
        } else if ans1 > ans2 {
            writeln!(stdout, "Bodya").unwrap();
        } else {
            writeln!(stdout, "Sasha").unwrap();
        }
    }
}

fn read<T: std::str::FromStr>(stdin: &std::io::Stdin, delimiter: &'static str) -> T {
    use std::io::BufRead;

    let stdin = stdin.lock();
    let line = stdin.lines().next().unwrap().unwrap();
    line.trim().parse::<T>().ok().expect("parse error")
}

fn read_vec<T: std::str::FromStr>(stdin: &std::io::Stdin, delimiter: &'static str) -> Vec<T> {
    use std::io::BufRead;

    let stdin = stdin.lock();
    let line = stdin.lines().next().unwrap().unwrap();
    line.split(delimiter).map(|s| s.trim().parse::<T>().ok().expect("parse error")).collect()
}