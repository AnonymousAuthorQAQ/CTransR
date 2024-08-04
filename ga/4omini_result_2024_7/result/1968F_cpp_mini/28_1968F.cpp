use std::io::{self, Read};
use std::cmp::Ordering;

struct Event {
    val: i64,
    index: usize,
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();
    let mut iterator = input.split_whitespace();

    let t: usize = iterator.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve(&mut iterator);
    }
}

fn solve<I: Iterator<Item = &'static str>>(iterator: &mut I) {
    let n: usize = iterator.next().unwrap().parse().unwrap();
    let q: usize = iterator.next().unwrap().parse().unwrap();

    let mut arr = Vec::new();
    for _ in 0..n {
        arr.push(iterator.next().unwrap().parse::<i64>().unwrap());
    }

    let mut x = vec![Event { val: 0, index: 0 }; n + 1];
    let mut x2 = vec![0; n + 1];

    x[0] = Event { val: 0, index: 0 };
    for i in 1..=n {
        x[i].val = x[i - 1].val ^ arr[i - 1];
        x[i].index = i;
    }

    for i in 0..=n {
        x2[i] = x[i].val;
    }

    x.sort_by(|a, b| {
        match a.val.cmp(&b.val) {
            Ordering::Equal => a.index.cmp(&b.index),
            other => other,
        }
    });

    for _ in 0..q {
        let l: usize = iterator.next().unwrap().parse().unwrap();
        let r: usize = iterator.next().unwrap().parse().unwrap();

        let c = x2[r] ^ x2[l - 1];
        if c == 0 {
            println!("YES");
            continue;
        }

        // Finding i1: the first index with value >= (c ^ x2[l-1])
        let target1 = c ^ x2[l - 1];
        let i1 = x.binary_search_by_key(&(target1, l), |e| (e.val, e.index)).unwrap_or_else(|x| x);

        if i1 >= n + 1 || x[i1].index > r || x[i1].val != target1 {
            println!("NO");
            continue;
        }

        // Finding i2 for the next condition
        let i2 = x.binary_search_by_key(&x2[l - 1], |e| e.val).unwrap_or_else(|x| x);
        
        if i2 >= n + 1 || x[i2].index > r || x[i2].val != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}