use std::cmp;

const N: usize = 2e5 + 3;

fn find(k: usize, p: &mut Vec<usize>) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        let parent = find(p[k], p);
        p[k] = parent;
        parent
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.split_whitespace().collect();
        let n: usize = parts[0].parse().unwrap();
        let x: usize = parts[1].parse().unwrap();
        let y: usize = parts[2].parse().unwrap();
        let a: Vec<char> = parts[3].chars().collect();
        
        let mut z = vec![0; n + 1];
        let mut p = vec![0; n + 2];
        let mut t = vec![0; n + 2];
        let mut ans = vec![0; n + 2];
        
        let mut j = 0;
        let mut c;
        
        p[0] = 0;
        t[0] = 0;
        ans[0] = 0;
        z[0] = 0;
        
        z[1] = n;
        
        for i in 2..=n {
            if j + z[j] > i {
                z[i] = cmp::min(z[i - j + 1], j + z[j] - i);
            }
            
            while a[i + z[i]] == a[1 + z[i]] {
                z[i] += 1;
            }
            
            if i + z[i] > j + z[j] {
                j = i;
            }
        }
        
        let mut t_sorted = (1..=n).collect::<Vec<usize>>();
        t_sorted.sort_by_key(|&i| z[i]);
        
        let mut k = 1;
        
        for i in 1..=n {
            c = 0;
            while k <= n && z[t_sorted[k - 1]] < i {
                p[t_sorted[k - 1]] = t_sorted[k - 1] + 1;
                k += 1;
            }
            
            for j in 1..=n {
                let mut index = j;
                while index <= n {
                    index = find(index + i, &mut p);
                    c += 1;
                }
            }
            
            ans[c] = i;
        }
        
        for i in (1..=n).rev() {
            ans[i] = cmp::max(ans[i], ans[i + 1]);
        }
        
        for i in x..=y {
            print!("{} ", ans[i]);
        }
        
        println!();
    }
}