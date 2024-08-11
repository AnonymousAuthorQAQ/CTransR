use std::io;
use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18;

fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

fn build(aa: &mut Vec<usize>, n: usize, st: &mut Vec<usize>) {
    let mut n_ = 1;
    while n_ < n {
        n_ <<= 1;
    }
    
    st.clear();
    st.resize(n_ * 2, 0);
    
    for i in 0..n {
        st[n_ + i] = aa[i];
    }
    
    let mut i = n_ as isize - 1;
    while i > 0 {
        st[i as usize] = max(st[(i << 1) | 0], st[(i << 1) | 1]);
        i -= 1;
    }
}

fn query(l: usize, a: usize, n_: usize, st: &Vec<usize>) -> usize {
    let mut l = l + n_;
    let mut r = n_ - 1;

    let mut l_org = l;

    while l_org <= r {
        if (l & 1) == 1 {
            if st[l] >= a {
                let mut cur_idx = l;
                while cur_idx < n_ {
			         cur_idx = if st[cur_idx << 1] >= a { cur_idx << 1 } else { cur_idx << 1 | 1 };
			    };
                
                return cur_idx - n_;
	        }
            l += 1;
	    }
        l >>= 1;
        r >>= 1;
    }
    
    n_
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();
    
    for _ in 0..t {
        let mut input_data = String::new();
        io::stdin().read_line(&mut input_data).unwrap();
        let mut iter = input_data.split_whitespace();

        let n: usize = iter.next().unwrap().parse().unwrap();
        let kl: usize = iter.next().unwrap().parse().unwrap();
        let kr: usize = iter.next().unwrap().parse().unwrap();
        let cc: &str = iter.next().unwrap();

        let cc_chars: Vec<char> = cc.trim().chars().collect();
        let mut zz: Vec<usize> = vec![0; n];
        let mut ans: Vec<usize> = vec![0; n + 1];

        let mut l = 0;
        let mut r = 0;
        
        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);
                
                let mut k = 0;
                while r < n && cc_chars[r] == cc_chars[r - l] {
                    r += 1;
                }
                
                zz[i] = r - l;
            }
        }

        let mut st: Vec<usize> = Vec::new();
        build(&mut zz, n, &mut st);

        ans.iter_mut().for_each(|x| *x = 0);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;
            loop {
                i = query(i + p, p, n_, &st);
                if i == n_ {
                    break;
                }
                k += 1;
            }
            ans[k] = cmp::max(ans[k], p);
        }

        for k in (1..n).rev() {
            ans[k] = cmp::max(ans[k], ans[k + 1]);
        }
        
        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
        println!();
    }
}