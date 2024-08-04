use std::io;
use std::io::prelude::*;
use std::collections::BinaryHeap;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut line = String::new();
    stdin.read_line(&mut line).expect("Failed to read line");
    let t = line.trim().parse::<i32>().expect("Invalid test cases count");

    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        let mut iter = line.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().expect("Invalid array size");
        let q: usize = iter.next().unwrap().parse().expect("Invalid queries count");

        let mut arr = Vec::with_capacity(n);
        line.clear();
        stdin.read_line(&mut line).expect("Failed to read line");
        for num_str in line.trim().split_whitespace() {
            arr.push(num_str.parse::<i32>().expect("Invalid array element"));
        }

        // Construct prefix XOR and sort
        let mut x = Vec::with_capacity(n + 1);
        x.push((0, 0));
        for (i, &val) in arr.iter().enumerate() {
            x.push((x[i].0 ^ val, i + 1));
        }
        x.sort_unstable();

        for _ in 0..q {
            line.clear();
            stdin.read_line(&mut line).expect("Failed to read line");
            let mut iter = line.trim().split_whitespace();
            let l: usize = iter.next().unwrap().parse().expect("Invalid range start");
            let r: usize = iter.next().unwrap().parse().expect("Invalid range end");

            let c = x[r].0 ^ x[l - 1].0;
            if c == 0 {
                writeln!(out, "YES").expect("Failed to write output");
                continue;
            }

            let i1 = match x.binary_search_by_key(&c ^ x[l - 1].0, |&(ref v, _)| v) {
                Ok(i) => i,
                Err(i) => i,
            };

            if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x[l - 1].0 {
                writeln!(out, "NO").expect("Failed to write output");
                continue;
            }

            let i2 = match x.binary_search_by_key(&x[l - 1].0, |&(ref v, _)| v) {
                Ok(i) => i,
                Err(i) => i,
            };

            if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x[l - 1].0 {
                writeln!(out, "NO").expect("Failed to write output");
            } else {
                writeln!(out, "YES").expect("Failed to write output");
            }
        }
    }
}