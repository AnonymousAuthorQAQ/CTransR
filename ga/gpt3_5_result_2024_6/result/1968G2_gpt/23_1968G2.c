use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

fn build(aa: &mut Vec<i32>, n: usize) -> Vec<i32> {
    let mut st = vec![0; N_ * 2];
    let mut n_ = 1;
    
    while n_ < n {
        n_ <<= 1;
    }
    
    st[n_..(n_ + n)].clone_from_slice(&aa);
    
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }
    
    st
}

fn query(l: usize, a: i32, st: &Vec<i32>) -> usize {
    let mut res = N_ - 1;
    let mut l_idx = l + N_;
    let mut r_idx = N_ + N_ - 1;

    while l_idx <= r_idx {
        if l_idx & 1 == 1 {
            if st[l_idx as usize] >= a {
                while l_idx < N_ {
                    l_idx = if st[l_idx << 1] >= a { l_idx << 1 } else { (l_idx << 1) + 1 };
                }
                return (l_idx - N_) as usize;
            }
            l_idx += 1;
        }
        l_idx /= 2;
        r_idx /= 2;
    }
    N_
}

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock();

    let mut input_line = String::new();
    input.read_line(&mut input_line).unwrap();
    let t: usize = input_line.trim().parse().unwrap();

    for _ in 0..t {
        input_line.clear();
       
        input.read_line(&mut input_line).unwrap();
        let mut parts = input_line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let kl: usize = parts.next().unwrap().parse().unwrap();
        let kr: usize = parts.next().unwrap().parse().unwrap();
        let cc = parts.next().unwrap().trim();
        let cc: Vec<char> = cc.chars().collect();

        let mut zz = vec![0; n];
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = r.max(i);

                while r < n && cc[r] == cc[r - l] {
                    r += 1;
                }
                zz[i] = r - l;
            }
        }

        let st = build(&mut zz, n);

        let mut ans = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            loop {
                i = query(i + p, p as i32, &st);

                if i == n {
                    break;
                }

                k += 1;
            }

            ans[k] = ans[k].max(p);
        }

        for k in (1..n).rev() {
            ans[k] = ans[k].max(ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }

        println!();
    }
}