use std::io::{self, BufRead};

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(aa: &Vec<i32>, n: usize) -> Vec<i32> {
    let mut st: Vec<i32> = vec![0; N_ * 2];
    let mut n_ = 1;
    
    while n_ < n {
        n_ <<= 1;
    }

    for i in 0..n {
        st[n_ + i] = aa[i];
    }

    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }

    st
}

fn query(l: usize, a: i32, n_: usize, st: &Vec<i32>) -> usize {
    let mut r = n_ - 1;
    let mut l = l;

    while l <= r {
		if (l & 1) == 1 {
			if st[l] >= a {
                let mut temp_l = l;
				while temp_l < n_ {
					temp_l = if st[temp_l << 1] >= a { temp_l << 1 } else { (temp_l << 1) + 1 };
				}
				return temp_l - n_;
			}
			l += 1;
		}
		l >>= 1;
		r >>= 1;
	}

	return n_;
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());

    let t: usize = lines.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let input: Vec<String> = lines.next().unwrap().split_whitespace().map(|s| s.to_string()).collect();

        let n: usize = input[0].parse().unwrap();
        let kl: usize = input[1].parse().unwrap();
        let kr: usize = input[2].parse().unwrap();
        let cc: String = input[3].clone();

        let mut zz: Vec<i32> = vec![0; n];

        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l as usize] < r {
                zz[i] = zz[i - l as usize];
            } else {
                l = i as usize;
                r = max(r, i);
                while r < n && cc.chars().nth(r).unwrap() == cc.chars().nth(r - l).unwrap() {
                    r += 1;
                }
                zz[i] = (r - l) as i32;
            }
        }

        let st = build(&zz, n);
        
        let mut ans: Vec<i32> = vec![0; n + 1];

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            while i < n {
                i = query(i + p, p as i32, n_, &st);
                if i < n_ {
                    k += 1;
                }
            }
            ans[k as usize] = max(ans[k as usize], p as i32);
        }

        for k in (1..=n - 1).rev() {
            ans[k] = max(ans[k], ans[k + 1]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}