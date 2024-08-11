use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> u64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = std::cmp::max(mx, cur + (k as u64) * (a[pos] as u64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Failed to parse test cases");

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut values = input.trim().split_whitespace().map(|s| s.parse().expect("Failed to parse value"));
        let n: usize = values.next().unwrap();
        let k: usize = values.next().unwrap();
        let ps1: usize = values.next().unwrap();
        let ps2: usize = values.next().unwrap();

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let p_values: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().expect("Failed to parse p value")).collect();
        p.extend(p_values.iter().map(|&x| x - 1));

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let a_values: Vec<usize> = input.trim().split_whitespace().map(|s| s.parse().expect("Failed to parse a value")).collect();
        a.extend(a_values.iter().cloned());

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