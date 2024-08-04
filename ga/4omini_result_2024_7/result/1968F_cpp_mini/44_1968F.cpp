use std::io;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
struct Pair {
    fs: i64,
    sc: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    
    let mut arr: Vec<i64> = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    arr.extend(input.split_whitespace().map(|x| x.parse::<i64>().unwrap()));
    
    let mut x: Vec<Pair> = vec![Pair { fs: 0, sc: 0 }];
    let mut x2: Vec<i64> = vec![0; n + 1];

    for i in 1..=n {
        x.push(Pair { 
            fs: x[i - 1].fs ^ arr[i - 1], 
            sc: i 
        });
    }
    
    for i in 0..=n {
        x2[i] = x[i].fs;
    }
    
    x.sort_unstable_by(|a, b| {
        a.fs.cmp(&b.fs).then(a.sc.cmp(&b.sc))
    });

    for _ in 0..q {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        
        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let mut i1 = x.binary_search_by(|pair| {
            let value = c ^ x2[l - 1];
            if pair.fs.cmp(&value) == Ordering::Equal {
                Ordering::Equal
            } else {
                pair.fs.cmp(&value)
            }
        }).unwrap_err();
        
        // Adjust the index since binary_search gives an insertion point
        if i1 == x.len() || x[i1].sc > r || x[i1].fs != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let mut i2 = x.binary_search_by(|pair| {
            if pair.fs.cmp(&x2[l - 1]) == Ordering::Equal {
                Ordering::Equal
            } else {
                pair.fs.cmp(&x2[l - 1])
            }
        }).unwrap_err();
        
        // Adjust the index since binary_search gives an insertion point
        if i2 == x.len() || x[i2].sc > r || x[i2].fs != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}