use std::io::{self, BufRead};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<u32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(u32, usize)> = vec![(0, 0)];
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    let mut x2: Vec<u32> = x.iter().map(|item| item.0).collect();
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

        let i1 = x.binary_search_by(|item| item.cmp(&(c ^ x2[l - 1], l)));
        if let Ok(i) = i1 {
            if i < n && x[i].1 <= r && x[i].0 == c ^ x2[l - 1] {
                let i2 = x.binary_search_by(|item| item.cmp(&(x2[l - 1], x[i].1 + 1)));
                if let Ok(j) = i2 {
                    if j < n && x[j].1 <= r && x[j].0 == x2[l - 1] {
                        println!("YES");
                        continue;
                    }
                }
            }
        }
        println!("NO");
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