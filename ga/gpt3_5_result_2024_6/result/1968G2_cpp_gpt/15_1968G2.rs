use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    let T: usize = input.trim().parse().unwrap();
    
    for _ in 0..T {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let params: Vec<usize> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = params[0];
        let x = params[1];
        let y = params[2];
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut a_chars = input.trim().chars().collect::<Vec<char>>();
        let mut a: Vec<char> = vec!['\0'];
        a.append(&mut a_chars);
        
        let mut z: Vec<usize> = vec![0; n + 1];
        let mut p: Vec<usize> = (0..=n+1).collect();
        let mut t: Vec<usize> = (0..=n+1).collect();
        let mut ans: Vec<usize> = vec![0; n + 1];
        
        let mut z_max = 0;
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = z_min(z[i-j+1], j+t[j]-i);
            }
            while a[i+z[i]] == a[1+z[i]] {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
            if z[i] > z_max {
                z_max = z[i];
            }
        }
        
        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by(|i, j| z[*i].cmp(&z[*j]));
        
        for i in 1..=n {
            let mut k = 1;
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }
            
            let mut j = 1;
            while j <= n {
                let mut idx = j;
                if idx + i > n {
                    idx = n + 1;
                } else {
                    idx = find(j + i, &mut p);
                
                    c += 1;
                    while idx <= n {
                        idx = find(idx + i, &mut p);
                        c += 1;
                    }
                }
                ans[c] = i;
            }
        }
        
        for i in (1..=n).rev() {
            ans[i] = std::cmp::max(ans[i], ans[i+1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        println!();
    }
}