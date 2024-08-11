use std::io::{self, BufRead};

fn solve() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut input_iter = input_line.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let q: usize = input_iter.next().unwrap().parse().unwrap();

    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    let arr: Vec<u32> = input_line.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(u32, usize)> = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        let xor_val = x[i - 1].0 ^ arr[i - 1];
        x.push((xor_val, i));
    }
    
    x.sort();

    for _ in 0..q {
        input_line.clear();
        io::stdin().read_line(&mut input_line).unwrap();
        let mut input_iter = input_line.split_whitespace();
        let l: usize = input_iter.next().unwrap().parse().unwrap();
        let r: usize = input_iter.next().unwrap().parse().unwrap();

        let c = x[r].0 ^ x[l - 1].0;
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x[l..].binary_search_by_key(&(c ^ x[l - 1].0), |&(val, _)| val).unwrap_or_else(|x| x);
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x[l - 1].0 {
            println!("NO");
            continue;
        }

        let i2 = x[x[i1].1..].binary_search_by_key(&x[l - 1].0, |&(val, _)| val).unwrap_or_else(|x| x);
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x[l - 1].0 {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}