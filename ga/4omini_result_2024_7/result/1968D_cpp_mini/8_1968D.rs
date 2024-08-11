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
    stdin.read_line(&mut input).expect("Failed to read input");
    let t: i32 = input.trim().parse().expect("Failed to parse input");

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");
        let mut nums = input.split_whitespace().map(|x| x.parse::<i32>().unwrap());
        let n = nums.next().unwrap() as usize;
        let k = nums.next().unwrap() as i64;
        let ps1 = nums.next().unwrap() as usize - 1;
        let ps2 = nums.next().unwrap() as usize - 1;

        let mut p = vec![0; n];
        let mut a = vec![0; n];

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");
        for (i, val) in input.split_whitespace().enumerate() {
            p[i] = val.parse::<usize>().unwrap() - 1; // Convert to 0-indexed
        }

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read input");
        for (i, val) in input.split_whitespace().enumerate() {
            a[i] = val.parse::<i64>().unwrap(); // Parse as i64
        }

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