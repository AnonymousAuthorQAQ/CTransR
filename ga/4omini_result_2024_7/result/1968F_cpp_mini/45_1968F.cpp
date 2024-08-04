use std::io::{self, BufRead};
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve(lines: &mut std::iter::Peekable<std::io::Lines<std::io::BufReader<io::StdinLock<'_>>>>) {
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
    let n = nums.next().unwrap();
    let q = nums.next().unwrap();

    let mut arr = vec![0; n];
    let mut idx_line = lines.next().unwrap().unwrap();
    arr.copy_from_slice(&mut idx_line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect::<Vec<_>>());

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    x[0] = (0, 0);
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i as i64);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort_by(|a, b| a.0.cmp(&b.0));

    for _ in 0..q {
        let line = lines.next().unwrap().unwrap();
        let mut lr = line.split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let l = lr.next().unwrap();
        let r = lr.next().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search_by(|&(key, _)| {
            if key < (c ^ x2[l - 1]) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).unwrap_err(); // this returns the insertion point for the search value

        if i1 == n + 1 || x[i1].1 > r as i64 || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }

        let i2 = x.binary_search_by(|&(key, _)| {
            if key < x2[l - 1] {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }).unwrap_err();

        if i2 == n + 1 || x[i2].1 > r as i64 || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}