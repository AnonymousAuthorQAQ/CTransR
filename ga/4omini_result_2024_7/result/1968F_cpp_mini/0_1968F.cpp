use std::io::{self, Write};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Pair {
    value: i64,
    index: usize,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Pair {}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.value != other.value {
            self.value.cmp(&other.value)
        } else {
            self.index.cmp(&other.index)
        }
    }
}

fn solve() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut arr = vec![0; n];
    input.clear();
    stdin.read_line(&mut input).unwrap();
    iter = input.split_whitespace();
    for i in 0..n {
        arr[i] = iter.next().unwrap().parse::<i64>().unwrap();
    }

    let mut prefix_xor = vec![Pair { value: 0, index: 0 }; n + 1];
    let mut prefix_xor_values = vec![0; n + 1];

    for i in 1..=n {
        prefix_xor[i].value = prefix_xor[i - 1].value ^ arr[i - 1];
        prefix_xor[i].index = i;
    }
    for i in 0..=n {
        prefix_xor_values[i] = prefix_xor[i].value;
    }

    prefix_xor.sort();

    let mut output = Vec::new();
    for _ in 0..q {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse::<usize>().unwrap();
        let r: usize = iter.next().unwrap().parse::<usize>().unwrap();

        let c = prefix_xor_values[r] ^ prefix_xor_values[l - 1];
        if c == 0 {
            output.push("YES".to_string());
            continue;
        }

        let target = c ^ prefix_xor_values[l - 1];
        let i1 = prefix_xor.binary_search(&Pair { value: target, index: l }).unwrap_err();
        if i1 == n + 1 || prefix_xor[i1].index > r || prefix_xor[i1].value != target {
            output.push("NO".to_string());
            continue;
        }

        let i2 = prefix_xor.binary_search(&Pair { value: prefix_xor_values[l - 1], index: prefix_xor[i1].index + 1 }).unwrap_err();
        if i2 == n + 1 || prefix_xor[i2].index > r || prefix_xor[i2].value != prefix_xor_values[l - 1] {
            output.push("NO".to_string());
        } else {
            output.push("YES".to_string());
        }
    }

    println!("{}", output.join("\n"));
}

fn main() {
    let stdin = io::stdin();
    let t: usize = {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };

    for _ in 0..t {
        solve();
    }
}