use std::io;

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let parts: Vec<&str> = input.trim().split(" ").collect();
    let n: usize = parts[0].parse().unwrap();
    let q: usize = parts[1].parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(usize, usize)> = vec![(0, 0); n+1];
    let mut x2: Vec<usize> = vec![0; n+1];

    x[0] = (0, 0);
    for i in 1..=n {
        x[i] = (x[i-1].0 ^ arr[i-1], i);
    }
    for i in 0..=n {
        x2[i] = x[i].0;
    }
    x.sort();

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let q_parts: Vec<usize> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let l = q_parts[0];
        let r = q_parts[1];

        let c = x2[r] ^ x2[l-1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by_key(&(c ^ x2[l-1]), |&(val, _)| val).unwrap_or_else(|x| x);
        if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l-1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by_key(&x2[l-1], |&(val, end)| end).unwrap_or_else(|x| x);
        if i2 == n+1 || x[i2].1 > r || x[i2].0 != x2[l-1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}