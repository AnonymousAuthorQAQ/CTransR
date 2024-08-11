use std::io::{self, BufRead};

fn solve() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let (n, q) = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
    let arr = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    
    let mut x = vec![(0, 0)];
    for (i, a) in arr.iter().enumerate() {
        x.push((x[i].0 ^ a, i + 1));
    }
    let mut x2 = x.iter().map(|&(a, _)| a).collect::<Vec<usize>>();
    x2.insert(0, 0);
    
    x.sort_unstable();
    
    for _ in 0..q {
        let (l, r) = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<usize>>()[..2].try_into().unwrap();
        let c = x2[r] ^ x2[l - 1];
        
        if c == 0 {
            println!("YES");
            continue;
        }
        
        let i1 = x.binary_search_by_key(&(c ^ x2[l - 1]), |&(a, _)| a).unwrap_or_else(|x| x);
        
        if i1 == n + 1 || x[i1].1 > r || x[i1].0 != (c ^ x2[l - 1]) {
            println!("NO");
            continue;
        }
        
        let i2 = x.binary_search_by_key(&(x2[l - 1], x[i1].1 + 1), |&(a, b)| (a, b)).unwrap_or_else(|x| x);
        
        if i2 == n + 1 || x[i2].1 > r || x[i2].0 != x2[l - 1] {
            println!("NO");
        } else {
            println!("YES");
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    let t = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        solve();
    }
}