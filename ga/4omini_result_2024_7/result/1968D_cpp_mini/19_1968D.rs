use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut current_pos = pos;
    let mut steps = k;

    while !vis[current_pos] && steps > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur + (steps as i64 * a[current_pos]));
        cur += a[current_pos];
        current_pos = p[current_pos];
        steps -= 1;
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
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();
        let ps1: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let ps2: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let p: Vec<usize> = input.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: Vec<i64> = input.split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect();

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