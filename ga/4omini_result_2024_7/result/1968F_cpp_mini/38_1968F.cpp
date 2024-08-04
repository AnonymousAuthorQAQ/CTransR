use std::io;
use std::cmp::Ordering;

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock();
    let reader = io::BufReader::new(input);
    let mut lines = reader.lines();

    let t: usize = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..t {
        solve(&mut lines);
    }
}

fn solve<I: Iterator<Item = io::Result<String>>>(lines: &mut I) {
    let first_line = lines.next().unwrap().unwrap();
    let nums: Vec<usize> = first_line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let n = nums[0];
    let q = nums[1];

    let arr: Vec<i32> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut x = vec![(0i32, 0); n + 1];
    let mut x2 = vec![0; n + 1];
    
    for i in 1..=n {
        x[i] = (x[i - 1].0 ^ arr[i - 1], i as usize);
    }

    for i in 0..=n {
        x2[i] = x[i].0;
    }

    x.sort_unstable();

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let lr: Vec<usize> = query_line.split_whitespace()
            .map(|x| x.parse().unwrap()).collect();
        
        let l = lr[0];
        let r = lr[1];

        let c = x2[r] ^ x2[l - 1]; // Note: Subtracting 1 for 0-based indexing

        if c == 0 {
            println!("YES");
            continue;
        }

        if let Ok(i1_idx) = x.binary_search(&(c ^ x2[l - 1], l)).get::<usize>() {
            let i1 = i1_idx.unwrap();
            if i1 >= n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
                println!("NO");
                continue;
            }

            if let Ok(i2_idx) = x.binary_search(&(x2[l - 1], x[i1].1 + 1)).get::<usize>() {
                let i2 = i2_idx.unwrap();
                if i2 >= n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
                    println!("NO");
                } else {
                    println!("YES");
                }
            } else {
                println!("NO");
            }
        } else {
            println!("NO");
        }
    }
}