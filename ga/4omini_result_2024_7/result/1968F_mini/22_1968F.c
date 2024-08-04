use std::io;
use std::time::{SystemTime, UNIX_EPOCH};

const N: usize = 200_000;

static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z *= 3;
        Z >> 1
    }
}

fn srand_() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    unsafe {
        Z = (now.as_secs() ^ now.subsec_micros() as u64) as u32 | 1;
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize) {
    if l >= r {
        return;
    }
    
    let i_ = aa[ii[l] as usize] ^ rand_() as u32 % (r - l) as u32;
    let (mut i, mut j, mut k) = (l, l, r);

    while j < k {
        let c = if aa[ii[j]] != i_ {
            aa[ii[j]] as i32 - i_ as i32
        } else {
            ii[j] as i32 - ii[i] as i32
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

    sort(ii, aa, l, i);
    sort(ii, aa, k, r);
}

fn search(aa: &[u32], ii: &[usize], a: u32, i: usize, n: usize) -> i32 {
    let (mut lower, mut upper) = (-1, n as i32 + 1);
    
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
    
    let stdin = io::stdin();
    let t: usize = {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        input.trim().parse().unwrap()
    };
    
    for _ in 0..t {
        let mut line = String::new();
        stdin.read_line(&mut line).unwrap();
        let mut parts = line.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        
        let mut aa = vec![0u32; n + 1];
        for i in 1..=n {
            let mut input = String::new();
            stdin.read_line(&mut input).unwrap();
            aa[i] = input.trim().parse::<u32>().unwrap();
            aa[i] ^= aa[i - 1];
        }
        
        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut ii, &aa, 0, n + 1);
        
        for _ in 0..q {
            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            let mut parts = line.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();
            let l = l - 1; // Convert to zero-based index
            
            let condition = aa[l] == aa[r] || ii[search(&aa, &ii, aa[r], l, n)] as usize + 1 < ii[search(&aa, &ii, aa[l], r, n)] as usize;
            if condition {
                println!("YES");
            } else {
                println!("NO");
            }
        }
    }
}