use std::io;
use std::io::{BufRead, Write};

fn solve(test_cases: &[(usize, Vec<i32>, Vec<(usize, usize)])]) {
    for (n, arr, queries) in test_cases {
        let mut x: Vec<(i32, usize)> = (0..*n + 1).map(|i| (i32::from(i), i)).collect();
        let mut x2: Vec<i32> = x.iter().map(|&(ref val, _)| val).collect();
        let mut sorted_x = x.clone();

        for (i, &val) in arr.iter().enumerate() {
            x[i + 1].0 ^= val;
            x2[i + 1] = x[i + 1].0;
        }

        sorted_x.sort_by(|a, b| {
            a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1))
        });

        for &(l, r) in queries.iter() {
            let c = x2[r] ^ x2[l - 1];
            if c == 0 {
                println!("YES");
                continue;
            }

            let i1 = match sorted_x.binary_search_by_key(&c ^ x2[l - 1], |&(ref val, _)| *val) {
                Ok(i) => i,
                Err(i) => i,
            };

            if i1 == *n || sorted_x[i1].1 > r || sorted_x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            let i2 = match sorted_x.binary_search_by_key(&x2[l - 1], |&(ref val, _)| *val) {
                Ok(i) => i,
                Err(i) => i,
            };

            if i2 == *n || sorted_x[i2].1 > r || sorted_x[i2].0 != x2[l - 1] {
                println!("NO");
            } else {
                println!("YES");
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut reader = io::BufReader::new(stdin.lock());
    let mut writer = io::BufWriter::new(stdout.lock());

    let t: usize = read!(reader, usize);
    let test_cases: Vec<(usize, Vec<i32>, Vec<(usize, usize)>)> = (0..t)
        .map(|_| {
            let n: usize = read!(reader, usize);
            let q: usize = read!(reader, usize);
            let arr: Vec<i32> = read!(reader, Vec<i32>);
            let queries: Vec<(usize, usize)> = (0..q)
                .map(|_| read!(reader, (usize, usize)))
                .collect();
            (n, arr, queries)
        })
        .collect();

    solve(&test_cases);
}

fn read<T: std::str::FromStr>() -> T {
    let stdin = io::stdin();
    let mut reader = io::BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().parse().ok().expect("Parse error")
}

fn read<T: std::str::FromStr + std::fmt::Debug, U: std::str::FromStr>(
    reader: &mut io::BufReader<io::StdinLock>,
) -> (T, U) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut iter = line.trim().split_whitespace();
    (
        iter.next().unwrap().parse().ok().expect("Parse error"),
        iter.next().unwrap().parse().ok().expect("Parse error"),
    )
}

fn read<T: std::str::FromStr>(reader: &mut io::BufReader<io::StdinLock>, n: usize) -> Vec<T> {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|s| s.parse().ok().expect("Parse error")).collect()
}