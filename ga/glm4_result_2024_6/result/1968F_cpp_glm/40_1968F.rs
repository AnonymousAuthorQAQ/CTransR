use std::io;
use std::io::{BufRead, Write};

fn solve() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = io::BufRead::new(stdin.lock());
    let mut writer = io::BufWriter::new(stdout.lock());

    let (n, q) = handle.read_tuple::<i32, i32>().expect("Invalid input");
    let mut arr = handle.read_array::<i32>(n as usize).expect("Invalid input");

    let mut x = Vec::with_capacity(n as usize + 1);
    let mut x2 = Vec::with_capacity(n as usize + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i as usize - 1], i));
    }
    for val in &x {
        x2.push(val.0);
    }
    x.sort_by(|a, b| a.0.cmp(&b.0));

    for _ in 0..q {
        let (l, r) = handle.read_tuple::<i32, i32>().expect("Invalid input");
        let c = x2[r as usize] ^ x2[l as usize - 1];
        if c == 0 {
            writeln!(writer, "YES").expect("Failed to write output");
            continue;
        }

        let i1 = match x.binary_search_by(|x| x.0.cmp(&c ^ x2[l as usize - 1])) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i1 == n as usize || x[i1].1 > r || x[i1].0 != (c ^ x2[l as usize - 1]) {
            writeln!(writer, "NO").expect("Failed to write output");
            continue;
        }

        let i2 = match x.binary_search_by(|x| x.0.cmp(&x2[l as usize - 1])) {
            Ok(i) => i,
            Err(i) => i,
        };

        if i2 == n as usize || x[i2].1 > r || x[i2].0 != x2[l as usize - 1] {
            writeln!(writer, "NO").expect("Failed to write output");
        } else {
            writeln!(writer, "YES").expect("Failed to write output");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = io::BufRead::new(stdin.lock());
    let t = handle.read_line().expect("Invalid input").trim().parse::<i32>().expect("Invalid input");
    for _ in 0..t {
        solve();
    }
}

fn read_tuple<T1, T2>(handle: &mut impl BufRead) -> Option<(T1, T2)>
where
    T1: std::str::FromStr,
    T1::Err: std::fmt::Debug,
    T2: std::str::FromStr,
    T2::Err: std::fmt::Debug,
{
    let mut line = String::new();
    if handle.read_line(&mut line).expect("Failed to read line") == 0 {
        return None;
    }
    let mut iter = line.trim().split_whitespace();
    Some((iter.next()?.parse::<T1>().expect("Invalid input"), iter.next()?.parse::<T2>().expect("Invalid input")))
}

fn read_array<T>(handle: &mut impl BufRead, size: usize) -> Option<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut line = String::new();
    if handle.read_line(&mut line).expect("Failed to read line") == 0 {
        return None;
    }
    line.trim().split_whitespace().map(|s| s.parse::<T>().expect("Invalid input")).collect()
}