use std::io;
use std::time::SystemTime;

const N: usize = 200_000;

fn rand_() -> u32 {
    static mut Z: u32 = 0;
    unsafe {
        Z = Z.wrapping_mul(3).wrapping_shr(1);
        Z
    }
}

fn srand_() {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    let secs = since_the_epoch.as_secs() as u32;
    let micros = since_the_epoch.subsec_micros() as u32;
    unsafe {
        Z = secs ^ micros | 1;
    }
}

fn sort(ii: &mut [usize], l: usize, r: usize) {
    if l < r {
        let i = l;
        let j = l;
        let k = r;
        let i_ = ii[l + rand_() as usize % (r - l)];
        while j < k {
            let c = if i_[j] != i_[i_] { i_[j] - i_[i_] } else { j - i_ };
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
        sort(ii, l, i);
        sort(ii, k, r);
    }
}

fn search(a: u32, i: usize, aa: &[u32], ii: &[usize]) -> isize {
    let mut lower = -1;
    let mut upper = (ii.len() as isize) + 1;
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
    let mut line = String::new();
    stdin.read_line(&mut line).unwrap();
    let t: usize = line.trim().parse().unwrap();
    
    for _ in 0..t {
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace();
        let n: usize = iter.next().unwrap().parse().unwrap();
        let q: usize = iter.next().unwrap().parse().unwrap();
        
        let mut aa = vec![0; n + 1];
        line.clear();
        stdin.read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace();
        for i in 1..=n {
            aa[i] = iter.next().unwrap().parse().unwrap();
            aa[i] ^= aa[i - 1];
        }
        
        let mut ii = (0..=n).collect::<Vec<usize>>();
        sort(&mut ii, 0, n + 1);
        
        for _ in 0..q {
            line.clear();
            stdin.read_line(&mut line).unwrap();
            let mut iter = line.trim().split_whitespace();
            let l: usize = iter.next().unwrap().parse().unwrap() - 1;
            let r: usize = iter.next().unwrap().parse().unwrap();
            
            let result = if aa[l] == aa[r] || ii[search(aa[r], l, &aa, &ii) as usize + 1] < ii[search(aa[l], r, &aa, &ii)] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}