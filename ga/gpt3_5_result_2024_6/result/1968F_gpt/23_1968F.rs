use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        Z as i32
    }
}

fn srand_() {
    let tv = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    unsafe {
        Z = (tv.as_secs() ^ tv.as_micros() as u64) as u32 | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    let mut l = l;
    let mut r = r;
    
    while l < r {
        let i = l;
        let j = l;
        let k = r;
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
        l = k;
    }
}

fn search(a: i32, i: usize, ii: &[usize], aa: &[i32]) -> i32 {
    let mut lower = -1;
    let mut upper = ii.len() as i32 + 1;
    
    while upper - lower > 1 {
        let h = (lower + upper) / 2;
        
        if aa[ii[h as usize]] < a || (aa[ii[h as usize]] == a && ii[h as usize] < i) {
            lower = h;
        } else {
            upper = h;
        }
    }
    
    lower
}

fn main() {
    srand_();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        
        let mut aa = vec![0; n + 1];
        let mut ii = (0..=n).collect::<Vec<usize>>();
        
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap() ^ aa[i - 1];
        }
        
        sort(&mut ii, 0, n + 1, &aa);
        
        for _ in 0..q {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            
            let result = if aa[l] == aa[r] || ii[(search(aa[r], l, &ii, &aa) + 1) as usize] < ii[search(aa[l], r, &ii, &aa) as usize] {
                "YES"
            } else {
                "NO"
            };
            
            println!("{}", result);
        }
    }
}