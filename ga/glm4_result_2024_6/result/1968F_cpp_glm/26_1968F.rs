use std::io;
use std::io::{Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let t = lines.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..t {
        let nq = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap());
        let (n, q) = (nq.next().unwrap(), nq.next().unwrap());

        let mut arr = Vec::new();
        for _ in 0..n {
            arr.push(lines.next().unwrap().parse::<usize>().unwrap());
        }

        let mut x = vec![(0, 0); n + 1];
        for i in 1..=n {
            x[i] = (x[i - 1].0 ^ arr[i - 1], i);
        }
        let x2: Vec<usize> = x.iter().map(|&(ref x, _)| x).collect();
        x.sort_by(|a, b| a.0.cmp(&b.0));

        for _ in 0..q {
            let lr = lines.next().unwrap().split_whitespace().map(|x| x.parse::<usize>().unwrap());
            let (l, r) = (lr.next().unwrap(), lr.next().unwrap());

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = match x.binary_search_by_key(&c ^ x2[l - 1], |&(ref x, _)| x) {
                Ok(i) => i,
                Err(_) => {
                    println!("NO");
                    continue;
                }
            };

            if x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = match x.binary_search_by_key(&x2[l - 1], |&(ref x, index)| (x, index + 1)) {
                Ok(i) => i,
                Err(_) => {
                    println!("NO");
                    continue;
                }
            };

            if x[i2].1 > r || x[i2].0 != x2[l - 1] {
                println!("NO");
                continue;
            }

            println!("YES");
        }
    }
}