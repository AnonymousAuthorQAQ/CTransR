use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|x| x.unwrap());

    let t: i32 = input.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let nk: Vec<i32> = input.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nk[0] as usize;
        let q = nk[1] as usize;

        let arr: Vec<i32> = input.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let mut x: Vec<(i32, usize)> = Vec::with_capacity(n + 1);
        x.push((0, 0));
        for i in 1..=n {
            x.push((x[i - 1].0 ^ arr[i-1], i));
        }
        
        x.sort();
        let mut x2: Vec<i32> = x.iter().map(|&val| val.0).collect();
        
        for _ in 0..q {
            let lr: Vec<usize> = input.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = lr[0];
            let r = lr[1];

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }
            
            let i1 = x.iter().position(|&val| val == (c ^ x2[l - 1], l)).unwrap_or(n + 1);

            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = x.iter().position(|&val| val == (x2[l-1], x[i1].1 + 1)).unwrap_or(n + 1);

            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}