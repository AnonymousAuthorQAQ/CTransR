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
    let mut iterator = stdin.lines();
    
    let t: usize = iterator.next().unwrap().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = iterator.next().unwrap().unwrap();
        let mut parameters = line.trim().split_whitespace().map(|x| x.parse::<i64>().unwrap());
        let n = parameters.next().unwrap() as usize;
        let k = parameters.next().unwrap();
        let ps1 = parameters.next().unwrap() as usize - 1;
        let ps2 = parameters.next().unwrap() as usize - 1;

        let p: Vec<usize> = iterator.next().unwrap().unwrap()
            .trim().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect();
            
        let a: Vec<i64> = iterator.next().unwrap().unwrap()
            .trim().split_whitespace()
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