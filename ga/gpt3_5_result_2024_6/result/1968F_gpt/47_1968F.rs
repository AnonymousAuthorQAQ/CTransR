use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        return (Z as i32);
    }
}

fn srand_() {
    unsafe {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                Z = (duration.as_secs() as u32) ^ duration.subsec_micros();
                if Z == 0 {
                    Z = 1;
                }
            }
            Err(_) => panic!("Error getting time since epoch"),
        }
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize, aa: &[i32]) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let irand = l + rand_() as usize % (r - l); 
        let mut i_ = ii[irand];
        while j < k {
            let c = if aa[ii[j]] != aa[i_] {aa[ii[j]] - aa[i_]} else {ii[j] as i32 - i_ as i32};
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

fn search(a: i32, i: usize, ii: &[usize], aa: &[i32]) -> isize {
    let mut lower = -1;
    let mut upper = (ii.len() + 1) as isize;
    while (upper - lower) > 1 {
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
    let mut input = String::new();
    let mut t = 0;

    srand_();
    std::io::stdin().read_line(&mut input).unwrap();
    let num_tests: i32 = input.trim().parse().unwrap();
    
    while t < num_tests {
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        
        input.clear();
        std::io::stdin().read_line(&mut input).unwrap();
        let aa: Vec<i32> = input.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
        
        let mut ii: Vec<usize> = (0..=n).collect();

        for i in 1..=n {
            let idx = i-1;
            let val = aa[idx];
            aa[idx] = aa[idx] ^ aa.get(idx.wrapping_sub(1)).unwrap_or(&0);
        }
        
        sort(&mut ii, 0, n+1, &aa);

        let mut query_counter = 0;
        while query_counter < q {
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.trim().split_whitespace();
            let l: i32 = iter.next().unwrap().parse().unwrap();
            let r: i32 = iter.next().unwrap().parse().unwrap();
            let l = l as usize;
            let r = r as usize;
            let l = l.checked_sub(1).unwrap_or(0);
            
            let index_l = search(aa[r], l, &ii, &aa) as usize;
            let index_r = search(aa[l], r, &ii, &aa) as usize;

            if index_l < index_r {
                println!("{}", "YES");
            } else {
                println!("{}", "NO");
            }

            query_counter += 1;
        }
        
        t += 1;
    }
}