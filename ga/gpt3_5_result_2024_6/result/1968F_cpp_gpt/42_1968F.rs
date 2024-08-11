use std::cmp::Ordering;
use std::io;

fn solve() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let mut input_parts = input_line.trim().split(' ');
    let n: usize = input_parts.next().unwrap().parse().unwrap();
    let q: usize = input_parts.next().unwrap().parse().unwrap();

    let mut arr: Vec<usize> = Vec::new();
    input_line.clear();
    io::stdin().read_line(&mut input_line).unwrap();
    for num in input_line.trim().split(' ') {
        arr.push(num.parse().unwrap());
    }

    let mut x: Vec<(usize, usize)> = Vec::with_capacity(n + 1);
    let mut x2: Vec<usize> = Vec::with_capacity(n + 1);
    x.push((0, 0));
    for i in 1..=n {
        x.push((x[i - 1].0 ^ arr[i - 1], i));
    }
    for i in 0..=n {
        x2.push(x[i].0);
    }
    x.sort();

    for _ in 0..q {
        let mut query_line = String::new();
        io::stdin().read_line(&mut query_line).unwrap();
        let mut query_parts = query_line.trim().split(' ');
        let l: usize = query_parts.next().unwrap().parse().unwrap();
        let r: usize = query_parts.next().unwrap().parse().unwrap();

        let c: usize = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        let i1 = match x.binary_search_by(|x| x.0.cmp(&(c ^ x2[l - 1]))) {
            Ok(idx) => idx,
            Err(_) => {
                println!("NO");
                continue;
            }
        };

        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != c ^ x2[l - 1] {
            println!("NO");
            continue;
        }

        let i2 = match x.binary_search_by(|x| x.0.cmp(&x2[l - 1])) {
            Ok(idx) => idx,
            Err(_) => {
                println!("NO");
                continue;
            }
        };

        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}