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
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let params: Vec<usize> = input.split_whitespace()
                                       .map(|x| x.parse().unwrap())
                                       .collect();
        let n = params[0];
        let k: i64 = params[1] as i64;
        let ps1 = params[2] - 1;
        let ps2 = params[3] - 1;

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let p: Vec<usize> = input.split_whitespace()
                                  .map(|x| x.parse::<usize>().unwrap() - 1)
                                  .collect();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let a: Vec<i64> = input.split_whitespace()
                                .map(|x| x.parse().unwrap())
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