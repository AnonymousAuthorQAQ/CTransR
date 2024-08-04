use std::io::{self, Read};

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let arr: Vec<i32> = input.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut x: Vec<(i32, usize)> = vec![(0, 0); n + 1];
    x[0] = (0, 0);

    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i);
    }

    let mut x2: Vec<i32> = x.iter().map(|(val, _)| *val).collect();
    x.sort();

    for _ in 0..q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();

        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.iter().position(|&(val, idx)| idx >= l && val == c ^ x2[l - 1]);

        match i1 {
            None => {
                println!("NO");
                continue;
            }
            Some(i) => {
                if x[i].1 > r {
                    println!("NO");
                    continue;
                }
            }
        }

        let i2 = x.iter().position(|&(val, idx)| idx >= x[i1.unwrap()].1 + 1 && val == x2[l - 1]);

        match i2 {
            None => {
                println!("NO");
            }
            Some(i) => {
                if x[i].1 > r {
                    println!("NO");
                } else {
                    println!("YES");
                }
            }
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