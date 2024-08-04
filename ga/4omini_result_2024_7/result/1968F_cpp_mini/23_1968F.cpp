use std::io::{self, Write}; // For reading input effectively
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Pair {
    fs: i64,
    sc: usize,
}

fn solve() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let mut iterator = input.lines();

    // Read n and q
    let first_line: Vec<usize> = iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = first_line[0];
    let q = first_line[1];

    // Read array
    let arr: Vec<i64> = iterator.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    // Create x and x2
    let mut x = vec![Pair { fs: 0, sc: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];
    for i in 1..=n {
        x[i].fs = x[i - 1].fs ^ arr[i - 1];
        x[i].sc = i;
    }
    for i in 0..=n {
        x2[i] = x[i].fs;
    }

    // Sort x by fs and then sc
    x.sort_unstable_by(|a, b| {
        match a.fs.cmp(&b.fs) {
            Ordering::Equal => a.sc.cmp(&b.sc),
            other => other,
        }
    });

    // Process queries
    let mut output = String::new();
    for _ in 0..q {
        let query_line: Vec<usize> = iterator.next().unwrap().unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let l = query_line[0];
        let r = query_line[1];

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            output.push_str("YES\n");
            continue;
        }

        // Binary search for the first index where fs == (c ^ x2[l-1]) and sc >= l
        let key1 = c ^ x2[l - 1];
        let mut lower_bound = match x.binary_search_by_key(&(key1, l), |p| (p.fs, p.sc)) {
            Ok(index) => index,
            Err(index) => index,
        };

        if lower_bound == n + 1 || x[lower_bound].sc > r || x[lower_bound].fs != (key1) {
            output.push_str("NO\n");
            continue;
        }

        // Binary search for the second condition
        let key2 = x2[l - 1];
        let mut lower_bound2 = match x.binary_search_by_key(&(key2, x[lower_bound].sc + 1), |p| (p.fs, p.sc)) {
            Ok(index) => index,
            Err(index) => index,
        };

        if lower_bound2 == n + 1 || x[lower_bound2].sc > r || x[lower_bound2].fs != key2 {
            output.push_str("NO\n");
        } else {
            output.push_str("YES\n");
        }
    }

    print!("{}", output);
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let iterator = input.lines();
    
    let mut iterator = iterator.filter_map(Result::ok);
    let t: usize = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}