use std::io::{self, BufRead};

fn calculate(p: &Vec<usize>, a: &Vec<i64>, mut pos: usize, mut k: i64) -> i64 {
    let n = p.len();
    let mut visited = vec![false; n];
    let mut max_score = 0;
    let mut current_score = 0;

    while !visited[pos] && k > 0 {
        visited[pos] = true;
        max_score = max_score.max(current_score + k * a[pos]);
        current_score += a[pos];
        pos = p[pos];
        k -= 1;
    }

    max_score
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let reader = handle.lines();

    let mut iter = reader.filter_map(Result::ok).map(|line| line.split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>());

    let t: i64 = iter.next().unwrap()[0];
    
    for _ in 0..t {
        let params = iter.next().unwrap();
        let n = params[0] as usize;
        let k = params[1];
        let ps1 = params[2] as usize - 1; // Convert to 0-indexed
        let ps2 = params[3] as usize - 1; // Convert to 0-indexed

        let p: Vec<usize> = iter.next().unwrap().iter().map(|&x| (x - 1) as usize).collect(); // Convert to 0-indexed
        let a: Vec<i64> = iter.next().unwrap();

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