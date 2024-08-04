use std::cmp;

const N: usize = 200003;

fn find(k: usize, p: &mut [usize]) -> usize {
    if k > p.len() || p[k] == k {
        k
    } else {
        p[k] = find(p[k], p);
        p[k]
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.trim().split('\n');

    let t: usize = lines.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line: Vec<usize> = lines
            .next()
            .unwrap()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        
        let n = line[0];
        let x = line[1];
        let y = line[2];
        let a = lines.next().unwrap().trim().chars().collect::<Vec<char>>();

        let mut z: Vec<usize> = vec![0; n + 3];
        let mut p: Vec<usize> = vec![0; n + 3];
        let mut t: Vec<usize> = vec![0; n + 3];
        let mut ans: Vec<usize> = vec![0; n + 3];
        
        let mut z_initial = 0;
        z[1] = n;

        for i in 1..=n {
            let mut j = 0;
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

        let mut t_sorted: Vec<usize> = (1..=n).collect();
        t_sorted.sort_by(|&i, &j| z[i].cmp(&z[j]));

        let mut k = 1;
        for i in 1..=n {
            let mut c = 0;
            while k <= n && z[t_sorted[k]] < i {
                p[t_sorted[k]] = t_sorted[k] + 1;
                k += 1;
            }

            let mut j = 1;
            while j <= n {
                j = find(j + i, &mut p);
                c += 1;
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