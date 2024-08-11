use std::collections::HashMap;

const N: usize = 2e5 as usize + 3;

fn find(k: usize, p: &mut [usize; N]) -> usize {
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
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let x: usize = iter.next().unwrap().parse().unwrap();
        let y: usize = iter.next().unwrap().parse().unwrap();
        let a: String = iter.next().unwrap().to_string();
        
        let mut p = [0; N];
        let mut t = [0; N];
        let mut ans = [0; N];
        let mut z = [0; N];
        let mut ans_map = HashMap::new();
        
        z[1] = n;
        
        for i in 2..=n {
            let mut j = 0;
            if j + z[j] > i {
                z[i] = z[i-j+1].min(j+z[j]-i);
            }
            while a.chars().nth(i-1+z[i]).unwrap() == a.chars().nth(1+z[i]-1).unwrap() {
                z[i] += 1;
            }
            if i + z[i] > j + z[j] {
                j = i;
            }
        }
        
        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));
        
        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k-1]] < i {
                p[t_sorted[k-1]] = t_sorted[k-1] + 1;
                k += 1;
            }
            let mut j = 1;
            while j <= n {
                j = find(j+i, &mut p);
                c += 1;
            }
            *ans_map.entry(c).or_insert(0) = i;
        }
        
        let mut output = Vec::new();
        for i in (1..=n).rev() {
            let val = *ans_map.get(&i).unwrap_or(&0);
            ans[i] = ans[i].max(val);
            output.push(ans[i]);
        }
        
        for i in x..=y {
            print!("{} ", output[i]);
        }
        println!();
    }
}