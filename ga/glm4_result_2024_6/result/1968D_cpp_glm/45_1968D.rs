use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<usize>, pos: usize, k: usize) -> i64 {
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; p.len()];
    while !vis[pos] && k > 0 {
        vis[pos] = true;
        mx = mx.max(cur + (k as i64) * (a[pos] as i64));
        cur += a[pos];
        pos = p[pos];
        k -= 1;
    }
    mx
}

fn main() {
    let mut t = 1;
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().is_empty() {
            break;
        }
        t = input.trim().parse().expect("Invalid input");
    }

    for _ in 0..t {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Invalid input");
        let k: usize = iter.next().unwrap().parse().expect("Invalid input");
        let ps1: usize = iter.next().unwrap().parse().expect("Invalid input");
        let ps2: usize = iter.next().unwrap().parse().expect("Invalid input");

        let mut p = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        for _ in 0..n {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let mut iter = input.trim().split_whitespace();
            p.push(iter.next().unwrap().parse().expect("Invalid input") - 1);
            a.push(iter.next().unwrap().parse().expect("Invalid input"));
        }

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