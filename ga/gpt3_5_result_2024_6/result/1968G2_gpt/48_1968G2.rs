use std::cmp;

const N: usize = 200000;
const N_: usize = 1 << 18; // N_ = pow2(ceil(log2(N)))

fn max(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn build(st: &mut [i32; N_ * 2], aa: &[i32], n: usize) {
    let mut n_ = 1;
    
    while n_ < n {
        n_ <<= 1;
    }
    
    st.iter_mut().enumerate().for_each(|(i, val)| {
        if i < n {
            *val = aa[i];
        } else {
            *val = 0;
        }
    });
    
    for i in (1..n_).rev() {
        st[i] = max(st[i << 1], st[(i << 1) + 1]);
    }
}

fn query(st: &[i32; N_ * 2], l: usize, a: i32, n_: usize) -> usize {
    let mut r = n_ - 1;
    let mut index = l + n_;

    while index <= r + n_ {
        if index & 1 == 1 {
            if st[index] >= a {
                let mut tmp = index;

                while tmp < n_ && st[(tmp << 1)] >= a {
                    tmp = if st[(tmp << 1)] >= a { (tmp << 1) } else { (tmp << 1) + 1 }
                }
                return tmp - n_;
            }
            
            index += 1;
        }

        index >>= 1;
    }
    
    n_
}

fn main() {
    let mut input_num = String::new();
    std::io::stdin().read_line(&mut input_num).unwrap();
  
    let t: i32 = input_num.trim().parse().unwrap();
  
    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let values: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
      
        let n = values[0] as usize;
        let kl = values[1] as usize;
        let kr = values[2] as usize;
        let cc = values[3].trim();

        let cc_chars: Vec<char> = cc.chars().collect();
      
        let mut zz = vec![0; n];
        let mut ans = vec![0; n+1];
      
        let mut l = 0;
        let mut r = 0;

        for i in 1..n {
            if i + zz[i - l] < r {
                zz[i] = zz[i - l];
            } else {
                l = i;
                r = cmp::max(r, i);

                while r < n && cc_chars[r] == cc_chars[r - l] {
                    r += 1;
                }
                
                zz[i] = r - l;
            }
            
        }

        let mut st = [0; N_ * 2];

        build(&mut st , &zz, n);

        for p in 1..=n {
            let mut k = 0;
            let mut i = 0;

            while i < n {
                i = query(&st, i + p, p as i32, n_);
                if i < n_ {
                    k += 1;
                }
            }

            ans[k] = cmp::max(ans[k], p);
        }

        for k in (1..=n - 1).rev() {
            ans[k - 1] = cmp::max(ans[k - 1], ans[k]);
        }

        for k in kl..=kr {
            print!("{} ", ans[k]);
        }
      
        println!();
      
    }
}