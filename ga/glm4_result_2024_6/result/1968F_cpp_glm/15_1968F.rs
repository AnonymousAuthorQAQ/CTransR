use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().parse::<i32>().unwrap();
    while t > 0 {
        let (n, q) = {
            let line = lines.next().unwrap().unwrap();
            let v: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            (v[0], v[1])
        };

        let arr: Vec<i32> = {
            let line = lines.next().unwrap().unwrap();
            line.split_whitespace().map(|s| s.parse().unwrap()).collect()
        };

        let mut x = vec![(0, 0)];
        for &a in arr.iter() {
            let last = x.last().unwrap();
            x.push((last.0 ^ a, last.1 + 1));
        }
        let x2: Vec<i32> = x.iter().map(|&t| t.0).collect();

        let mut x_sorted = x.clone();
        x_sorted.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        for _ in 0..q {
            let (l, r) = {
                let line = lines.next().unwrap().unwrap();
                let v: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
                (v[0], v[1])
            };

            let c = x2[(r - 1) as usize] ^ x2[(l - 1) as usize];
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = match x_sorted.binary_search_by(|&(ref a, ref b)| {
                if *a == c ^ x2[(l - 1) as usize] {
                    b.cmp(&l)
                } else {
                    a.cmp(&(c ^ x2[(l - 1) as usize]))
                }
            }) {
                Ok(i) => i,
                Err(_) => {
                    println!("NO");
                    continue;
                }
            };

            let i2 = match x_sorted.binary_search_by(|&(ref a, ref b)| {
                if *a == x2[(l - 1) as usize] {
                    b.cmp(&(x[i1].1 + 1))
                } else {
                    a.cmp(&(x2[(l - 1) as usize]))
                }
            }) {
                Ok(i) => i,
                Err(_) => {
                    println!("NO");
                    continue;
                }
            };

            println!("YES");
        }

        t -= 1;
    }
}