use std::io::{self, BufRead};
use std::cmp::Ordering;

#[derive(Debug)]
struct PrefixXOR {
    value: i64,
    index: usize,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let first_line = lines.next().unwrap().unwrap();
        let mut parts = first_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        let mut arr = Vec::new();
        let arr_line = lines.next().unwrap().unwrap();
        for num in arr_line.split_whitespace() {
            arr.push(num.parse::<i64>().unwrap());
        }

        let mut x: Vec<PrefixXOR> = vec![PrefixXOR { value: 0, index: 0 }];
        let mut x2: Vec<i64> = vec![0; n + 1];

        for i in 1..=n {
            x.push(PrefixXOR { 
                value: x[i - 1].value ^ arr[i - 1], 
                index: i 
            });
            x2[i] = x[i].value;
        }

        // Sort based on value and index
        x.sort_by(|a, b| {
            if a.value == b.value {
                a.index.cmp(&b.index)
            } else {
                a.value.cmp(&b.value)
            }
        });

        for _ in 0..q {
            let query_line = lines.next().unwrap().unwrap();
            let mut parts = query_line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            let c = x2[r] ^ x2[l - 1];

            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = x.binary_search_by(|p| {
                let key = c ^ x2[l - 1];
                if p.value == key && p.index >= l {
                    Ordering::Equal
                } else if p.value < key {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }).unwrap_or_else(|x| x);

            if i1 == x.len() || x[i1].index > r || x[i1].value != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = x.binary_search_by(|p| {
                if p.value == x2[l - 1] && p.index > x[i1].index {
                    Ordering::Equal
                } else if p.value < x2[l - 1] || p.index <= x[i1].index {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            }).unwrap_or_else(|x| x);

            if i2 == x.len() || x[i2].index > r || x[i2].value != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}