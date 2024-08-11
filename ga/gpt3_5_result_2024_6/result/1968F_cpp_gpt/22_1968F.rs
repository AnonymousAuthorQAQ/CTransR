use std::cmp::Ordering;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let line: String = lines.next().unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut arr: Vec<i32> = Vec::new();
        let line: String = lines.next().unwrap();
        arr.extend(line.split_whitespace().map(|x| x.parse().unwrap()));

        let mut x: Vec<(i32, usize)> = vec![(0, 0); n+1];
        x[0] = (0, 0);
        for i in 1..=n {
            x[i] = (x[i-1].0 ^ arr[i-1], i);
        }

        let mut x2: Vec<i32> = Vec::new();
        x2.extend(x.iter().map(|&a| a.0));
        x.sort();

        for _ in 0..q {
            let line: String = lines.next().unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();

            let c = x2[r] ^ x2[l-1];
            if c == 0 {
                println!("YES");
                continue;
            }
            match x.binary_search_by(|a| a.cmp(&(c ^ x2[l-1]))) {
                Ok(i1) => {
                    if i1 == n+1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l-1]) {
                        println!("NO");
                        continue;
                    }
                    match x[i1+1..].binary_search_by(|a| a.1.cmp(&r)) {
                        Ok(i2) => {
                            if x[i1+i2+1].0 == x2[l-1] {
                                println!("YES");
                            } else {
                                println!("NO");
                            }
                        }
                        _ => println!("NO")
                    }
                }
                _ => println!("NO")
            }
        }
    }
}