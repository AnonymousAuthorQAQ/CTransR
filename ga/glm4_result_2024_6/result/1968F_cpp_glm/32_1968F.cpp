use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let mut t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let mut line = lines.next().unwrap().unwrap();
        let mut split = line.split_whitespace();
        let n = split.next().unwrap().parse::<i32>().unwrap();
        let q = split.next().unwrap().parse::<i32>().unwrap();

        let arr: Vec<i32> = (0..n).map(|_| split.next().unwrap().parse::<i32>().unwrap()).collect();

        let mut x: Vec<(i32, i32)> = Vec::with_capacity(n + 1);
        x.push((0, 0));
        for (i, &val) in arr.iter().enumerate() {
            x.push((x[i].0 ^ val, i as i32 + 1));
        }

        let mut x2 = x.clone();
        x2.sort_unstable();

        while q > 0 {
            let mut line = lines.next().unwrap().unwrap();
            let mut split = line.split_whitespace();
            let l = split.next().unwrap().parse::<i32>().unwrap();
            let r = split.next().unwrap().parse::<i32>().unwrap();

            let c = x2[r as usize].0 ^ x2[(l - 1) as usize].0;
            if c == 0 {
                println!("YES");
            } else {
                let i1 = match x2.binary_search_by_key(&c ^ x2[(l - 1) as usize].0, |x| x.0) {
                    Ok(i) => i,
                    Err(i) => i,
                };
                if i1 == n as usize || x2[i1].1 > r || x2[i1].0 != (c ^ x2[(l - 1) as usize].0) {
                    println!("NO");
                } else {
                    let i2 = match x2.binary_search_by_key(&x2[(l - 1) as usize].0, |x| x.0) {
                        Ok(i) => i,
                        Err(i) => i,
                    };
                    if i2 == n as usize || x2[i2].1 > r || x2[i2].0 != x2[(l - 1) as usize].0 {
                        println!("NO");
                    } else {
                        println!("YES");
                    }
                }
            }
            q -= 1;
        }

        t -= 1;
    }
}