use std::io::{self, BufRead};
use std::cmp::Ordering;

struct Pair {
    fs: i64,
    sc: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    
    let mut idx = 0;
    let t: usize = lines[idx].parse().unwrap();
    idx += 1;

    let mut results = Vec::new();

    for _ in 0..t {
        let (n, q) = {
            let nums: Vec<_> = lines[idx].split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
            (nums[0], nums[1])
        };
        idx += 1;

        let arr: Vec<_> = lines[idx].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
        idx += 1;

        let mut x: Vec<Pair> = vec![Pair { fs: 0, sc: 0 }];
        for i in 1..=n {
            let pair = Pair {
                fs: x[i - 1].fs ^ arr[i - 1],
                sc: i,
            };
            x.push(pair);
        }
        
        let x2: Vec<i64> = x.iter().map(|p| p.fs).collect();
        x.sort_unstable_by(|a, b| {
            if a.fs == b.fs {
                a.sc.cmp(&b.sc)
            } else {
                a.fs.cmp(&b.fs)
            }
        });

        for _ in 0..q {
            let (l, r) = {
                let nums: Vec<_> = lines[idx].split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect();
                (nums[0], nums[1])
            };
            idx += 1;

            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                results.push("YES".to_string());
                continue;
            }

            let mut i1 = x.binary_search(&Pair { fs: c ^ x2[l - 1], sc: l }).unwrap_err();
            if i1 == x.len() || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
                results.push("NO".to_string());
                continue;
            }

            i1 += 1; // Move to the next index
            let mut i2 = x.binary_search(&Pair { fs: x2[l - 1], sc: x[i1 - 1].sc + 1 }).unwrap_err();
            if i2 == x.len() || x[i2].sc > r || x[i2].fs != x2[l - 1] {
                results.push("NO".to_string());
            } else {
                results.push("YES".to_string());
            }
        }
    }
    
    // Output all results at once
    println!("{}", results.join("\n"));
}