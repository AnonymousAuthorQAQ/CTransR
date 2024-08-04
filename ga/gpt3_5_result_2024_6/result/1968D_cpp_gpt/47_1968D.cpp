use std::io::{self, Read};
use std::collections::HashSet;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, mut pos: usize, mut k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = HashSet::new();
    
    while !vis.contains(&pos) && k > 0 {
        vis.insert(pos);
        mx = mx.max(cur + (k as u64 * a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let nk: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nk[0];
        let k = nk[1];
        let ps: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let ps1 = ps[0];
        let ps2 = ps[1];
        let p: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap() - 1).collect();
        let a: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();

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