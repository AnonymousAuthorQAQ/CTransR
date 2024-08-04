use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Clone, Copy)]
struct Pair {
    first: i64, // represent the XOR value
    second: usize, // represent the index
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut input = reader.lines().map(|line| line.unwrap());

    let t: usize = input.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let nums: Vec<i64> = input.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        let n = nums[0] as usize;
        let q = nums[1] as usize;

        let arr: Vec<i64> = input.next().unwrap().split_whitespace()
            .map(|x| x.parse().unwrap()).collect();

        // Calculate prefix XORs
        let mut x: Vec<Pair> = vec![Pair { first: 0, second: 0 }];
        let mut x2: Vec<i64> = vec![0; n + 1];
        
        for i in 0..n {
            x.push(Pair {
                first: x[i].first ^ arr[i],
                second: i + 1,
            });
            x2[i + 1] = x[i + 1].first;
        }

        // Sort using a custom comparator
        x.sort_by(|a, b| {
            if a.first != b.first {
                a.first.cmp(&b.first)
            } else {
                a.second.cmp(&b.second)
            }
        });

        for _ in 0..q {
            let query: Vec<usize> = input.next().unwrap().split_whitespace()
                .map(|x| x.parse().unwrap()).collect();
            let l = query[0];
            let r = query[1];

            let c = x2[r] ^ x2[l - 1];

            if c == 0 {
                println!("YES");
                continue;
            }

            let mut i1 = x.binary_search(&Pair {
                first: c ^ x2[l - 1],
                second: l,
            });

            match i1 {
                Ok(idx) => {
                    if x[idx].second > r {
                        println!("NO");
                        continue;
                    }
                }
                Err(idx) => {
                    if idx == x.len() || x[idx].second > r || x[idx].first != (c ^ x2[l - 1]) {
                        println!("NO");
                        continue;
                    }
                }
            }

            let mut i2 = x.binary_search(&Pair {
                first: x2[l - 1],
                second: x[i1.unwrap_or_default()].second + 1,
            });

            match i2 {
                Ok(idx) => {
                    if x[idx].second > r || x[idx].first != x2[l - 1] {
                        println!("NO");
                    } else {
                        println!("YES");
                    }
                }
                Err(idx) => {
                    if idx == x.len() || x[idx].second > r || x[idx].first != x2[l - 1] {
                        println!("NO");
                    } else {
                        println!("YES");
                    }
                }
            }
        }
    }
}