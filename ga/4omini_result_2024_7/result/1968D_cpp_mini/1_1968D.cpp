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
        let params: Vec<i64> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let n = params[0] as usize;
        let k = params[1];
        let ps1 = (params[2] - 1) as usize; // 0-based index
        let ps2 = (params[3] - 1) as usize; // 0-based index
        
        let mut p = vec![0; n];
        let mut a = vec![0; n];
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        p = input.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect(); // 0-based index
        
        input.clear();
        stdin.read_line(&mut input).unwrap();
        a = input.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect();
        
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