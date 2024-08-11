use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let nm: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nm[0];
        let q = nm[1];

        let arr: Vec<u32> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut x: Vec<(u32, usize)> = vec![(0, 0)];
        for i in 1..=n {
            x.push((x[i-1].0 ^ arr[i-1], i));
        }
        let mut x2: Vec<u32> = x.iter().map(|&(val, _)| val).collect();
        x.sort();

        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = lr[0];
            let r = lr[1];

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(val, _)| val).unwrap_or_else(|x| x);
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x2[l - 1] {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by_key(&x2[l - 1], |&(val, idx)| idx).unwrap_or_else(|x| x);
            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}