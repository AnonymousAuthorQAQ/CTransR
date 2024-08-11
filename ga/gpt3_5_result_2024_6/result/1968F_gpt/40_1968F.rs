use std::io::{self, BufRead};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    let tv = match std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH) {
        Ok(duration) => duration.as_secs() as u32 ^ duration.as_nanos() as u32 | 1,
        Err(_) => 1,
    };
    
    unsafe { Z = tv };
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {  
    if l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = aa[ii[l + rand_() as usize % (r - l)] as usize];
        
        while j < k {
            let c = if aa[ii[j] as usize] != aa[i_] {
                aa[ii[j] as usize] - aa[i_]
            } else {
                ii[j] as i32 - i_ as i32
            };
            
            if c == 0 {
                j += 1;
            } else if c < 0 {
                ii.swap(i, j);
                i += 1;
                j += 1;
            } else {
                k -= 1;
                ii.swap(j, k);
            }
        }
        
        sort(ii, l, i, aa);
        sort(ii, k, r, aa);
    }
}

fn search(a: i32, i: usize, ii: &[usize], aa: &[i32]) -> i32 {
    let mut lower = -1;
    let mut upper = ii.len() as i32;

    while upper - lower > 1 {
        let h = (lower + upper) / 2;

        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i as usize) {
            lower = h;
        } else {
            upper = h;
        }
    }
    
    lower
}

fn main() {
    srand_();

    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().map(|x| x.unwrap());
    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let arr_info: Vec<usize> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let n = arr_info[0];
        let q = arr_info[1];

        let aa: Vec<i32> = lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let mut aa_xor: Vec<i32> = vec![0; n + 1];
        for i in 1..=n {
            aa_xor[i] = aa_xor[i - 1] ^ aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, 0, n + 1, &aa_xor);

        for _ in 0..q {
            let query: Vec<usize> = lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let l = query[0] - 1;
            let r = query[1];

            let result = if aa_xor[l] == aa_xor[r] || ii[search(aa_xor[r], l, &ii, &aa_xor) as usize + 1] < ii[search(aa_xor[l], r, &ii, &aa_xor) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}