use std::io::{self, BufRead};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
    }
    return Z as i32;
}

fn srand_() {
    let tv = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap();

    unsafe {
        Z = tv.as_secs() as u32 ^ tv.subsec_micros() | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    let mut l = l;
    let mut r = r;
    
    while l < r {
        let mut i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[l + (rand_().abs() as usize) % (r - l)];
        let mut tmp: usize;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i]; 
                ii[i] = ii[j]; 
                ii[j] = tmp;
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j]; 
                ii[j] = ii[k]; 
                ii[k] = tmp;
            }
        }
        sort(ii, l, i, aa);
        l = k;
    }
}

fn search(a: i32, i: usize, ii: &[usize], aa: &[i32]) -> isize {
    let mut lower = -1;
    let mut upper = (aa.len() + 1) as isize;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    return lower;
}

fn main() {
    srand_();
    
    let stdin = io::stdin();
    let mut input_iter = stdin.lock().lines().map(Result::unwrap);
    
    let t: usize = input_iter.next().unwrap().parse().unwrap();
    
    for _ in 0..t {
        let nq: Vec<usize> = input_iter.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        let n = nq[0];
        let q = nq[1];
        
        let mut aa: Vec<i32> = vec![0; n + 1];
        
        let aa_input: Vec<i32> = input_iter.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
        for i in 1..=n {
            aa[i] = aa[i - 1] ^ aa_input[i - 1];
        }
        
        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1, &aa);
        
        for _ in 0..q {
            let lr: Vec<usize> = input_iter.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
            let l = lr[0] - 1;
            let r = lr[1];
            
            let search_l = search(aa[r], l, &ii, &aa);
            let search_r = search(aa[l], r, &ii, &aa);
            
            if aa[l] == aa[r] || ii[(search_r + 1) as usize] < ii[(search_l) as usize] {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}