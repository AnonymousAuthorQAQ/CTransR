use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let nq: Vec<usize> = lines.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];

        let arr: Vec<usize> = lines.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let mut x: Vec<(usize, usize)> = vec![(0, 0)];
        for i in 0..n {
            x.push((x[i].0 ^ arr[i], i + 1));
        }
        let mut x2: Vec<usize> = x.iter().map(|&(a, _)| a).collect();
        x.sort();

        for _ in 0..q {
            let lr: Vec<usize> = lines.next().unwrap().trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = lr[0];
            let r = lr[1];
            let c = x2[r] ^ x2[l - 1];

            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(a, _)| a).unwrap_or_else(|x| x);
            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x2[l - 1] {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by_key(&x2[l - 1], |&(a, b)| b).unwrap_or_else(|x| x);
            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}