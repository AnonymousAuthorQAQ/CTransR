use std::io;

fn calculate(p: &Vec<usize>, a: &Vec<i64>, pos: usize, k: usize) -> i64 {
    let n = p.len();
    let mut mx = 0;
    let mut cur = 0;
    let mut vis = vec![false; n];
    let mut k = k;

    let mut current_pos = pos;
    
    while !vis[current_pos] && k > 0 {
        vis[current_pos] = true;
        mx = mx.max(cur + (k as i64 * a[current_pos]));
        cur += a[current_pos];
        current_pos = p[current_pos];
        k -= 1;
    }
    
    mx
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).expect("Failed to read line");
    let t: usize = input.trim().parse().expect("Please type a number!");
    
    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        let mut parameters = input.split_whitespace();
        let n: usize = parameters.next().unwrap().parse().expect("Failed to parse number");
        let k: usize = parameters.next().unwrap().parse().expect("Failed to parse number");
        let ps1: usize = parameters.next().unwrap().parse().expect("Failed to parse number") - 1;
        let ps2: usize = parameters.next().unwrap().parse().expect("Failed to parse number") - 1;

        let mut p = Vec::new();
        let mut a = Vec::new();

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        for num in input.split_whitespace() {
            p.push(num.parse::<usize>().expect("Parse error") - 1);
        }

        input.clear();
        stdin.read_line(&mut input).expect("Failed to read line");
        for num in input.split_whitespace() {
            a.push(num.parse::<i64>().expect("Parse error"));
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