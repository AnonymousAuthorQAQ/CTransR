use std::io;

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
    let mut iter = stdin.lines();
    let t: usize = iter.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        // Reading inputs
        let first_line = iter.next().unwrap().unwrap();
        let mut params = first_line.split_whitespace();
        let n: usize = params.next().unwrap().parse().unwrap();
        let k: i64 = params.next().unwrap().parse().unwrap();
        let ps1: usize = params.next().unwrap().parse::<usize>().unwrap() - 1; // converting to 0-indexed
        let ps2: usize = params.next().unwrap().parse::<usize>().unwrap() - 1; // converting to 0-indexed

        let mut p: Vec<usize> = Vec::with_capacity(n);
        let mut a: Vec<i64> = Vec::with_capacity(n);

        // Read the p array
        let p_line = iter.next().unwrap().unwrap();
        for val in p_line.split_whitespace() {
            p.push(val.parse::<usize>().unwrap() - 1); // converting to 0-indexed
        }

        // Read the a array
        let a_line = iter.next().unwrap().unwrap();
        for val in a_line.split_whitespace() {
            a.push(val.parse::<i64>().unwrap());
        }

        // Calculate scores
        let ans1 = calculate(&p, &a, ps1, k);
        let ans2 = calculate(&p, &a, ps2, k);

        // Determine the result
        if ans1 == ans2 {
            println!("Draw");
        } else if ans1 > ans2 {
            println!("Bodya");
        } else {
            println!("Sasha");
        }
    }
}