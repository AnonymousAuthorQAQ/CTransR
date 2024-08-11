use std::io::{self, BufRead};
use std::cmp::Ordering;

fn solve() {
    let stdin = io::stdin(); 
    let input = stdin.lock().lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let mut index = 0;

    let first_line = input[index].split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
    let n = first_line[0];
    let q = first_line[1];
    index += 1;

    let arr: Vec<i32> = input[index].split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
    index += 1;

    let mut x = vec![(0, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 0..n {
        x[i + 1].0 = x[i].0 ^ arr[i];
        x[i + 1].1 = i + 1;
    }
    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort_unstable();

    for _ in 0..q {
        let range = input[index].split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect::<Vec<_>>();
        index += 1;
        let l = range[0];
        let r = range[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = x.binary_search(&(c ^ x2[l - 1], l)).unwrap_or_else(|e| e);
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).unwrap_or_else(|e| e);
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let t: usize = stdin.lock().lines().next().unwrap().unwrap().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}