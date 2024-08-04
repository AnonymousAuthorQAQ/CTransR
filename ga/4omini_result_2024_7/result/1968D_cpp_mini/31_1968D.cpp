use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: usize) -> i64 {
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; p.len()];
    let mut pos = pos;
    let mut k = k;

    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * a[pos]);
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap(); 
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut params = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let n = params.next().unwrap();
        let k = params.next().unwrap();
        let ps1 = params.next().unwrap() - 1; // converting to 0-indexed
        let ps2 = params.next().unwrap() - 1; // converting to 0-indexed

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let p: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect(); // converting to 0-indexed

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();

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