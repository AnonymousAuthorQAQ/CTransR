use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(i32, usize)> = vec![(0, 0)];
    let mut x2: Vec<i32> = vec![0; n + 1];

    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }

    x2 = x.iter().map(|&(_, x)| x as i32).collect();
    x.sort();

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();

        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.iter().position(|&(x, _)| x == c ^ x2[l - 1])
            .unwrap_or(n + 1);

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x2[l - 1] {
            println!("NO");
            continue;
        }

        let i2 = x.iter().position(|&(x, i)| x == x2[l - 1] && i > x[i1].1)
            .unwrap_or(n + 1);

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}