use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = input.next().unwrap().trim().parse().unwrap();
    
    for _ in 0..t {
        let line = input.next().unwrap();
        let parts: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = parts[0];
        let kl = parts[1];
        let kr = parts[2];
        let cc = input.next().unwrap().trim().to_string();
        
        let zz: Vec<usize> = compute_z_array(&cc);

        let mut ans = vec![0; n + 1];
        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while let Some(index) = query(i + p, p, &zz) {
                k += 1;
                i = index;
            }
            ans[k] = ans[k].max(p);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn compute_z_array(s: &str) -> Vec<usize> {
    let n = s.len();
    let mut zz = vec![0; n];
    let mut l = 0;
    let mut r = 0;

    for i in 1..n {
        if i + zz[i - l] < r {
            zz[i] = zz[i - l];
        } else {
            l = i;
            r = r.max(i);
            while r < n && s.chars().nth(r) == s.chars().nth(r - l) {
                r += 1;
            }
            zz[i] = r - l;
        }
    }

    zz
}

fn query(mut l: usize, p: usize, zz: &Vec<usize>) -> Option<usize> {
    let n = zz.len() - 1;
    let mut r = n - 1;

    while l <= r {
        let mut index = l;
        if l & 1 == 1 {
            if zz[l] >= p {
                while index < n {
                    index = if zz[index * 2] >= p { index * 2 } else { index * 2 + 1 };
                }
                return Some(index - n);
            }
            l += 1;
        }
    }
    None
}