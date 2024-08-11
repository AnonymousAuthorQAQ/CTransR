use std::io;
use std::time::SystemTime;

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> u32 {
    unsafe {
        Z = Z.wrapping_mul(3);
        Z >> 1
    }
}

fn srand_() {
    let now = SystemTime::now();
    let since_epoch = now.duration_since(std::time::UNIX_EPOCH).expect("Time went backwards");
    let secs = since_epoch.as_secs() as u32;
    let micros = since_epoch.subsec_micros();
    unsafe {
        Z = (secs ^ micros) | 1;
    }
}

fn sort(ii: &mut [usize], aa: &[u32], l: usize, r: usize) {
    if l >= r {
        return;
    }
    
    let mut i = l;
    let mut j = l;
    let mut k = r;
    let i_ = ii[l + (rand_() % (r - l) as usize)];
    
    while j < k {
        let c = if aa[ii[j]] != aa[i_] {
            aa[ii[j]] as i32 - aa[i_] as i32
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

fn search(aa: &[u32], ii: &[usize], i: usize, a: u32) -> i32 {
    let mut lower = -1;
    let mut upper = ii.len() as i32 - 1;

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
    let mut input = String::new();
    
    stdin.read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let mut parts = input.split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();
        
        let mut aa = vec![0u32; n + 1];
        let mut ii: Vec<usize> = (0..=n).collect();

        input.clear();
        stdin.read_line(&mut input).unwrap();
        let values: Vec<u32> = input.split_whitespace().map(|s| s.parse().unwrap()).collect();

        for i in 1..=n {
            aa[i] = values[i - 1] ^ aa[i - 1];
        }

        sort(&mut ii, &aa, 0, n + 1);

        for _ in 0..q {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            let mut parts = input.split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap() - 1;
            let r: usize = parts.next().unwrap().parse().unwrap();
            
            let left_search = search(&aa, &ii, l, aa[r]);
            let right_search = search(&aa, &ii, r, aa[l]);
            let answer = if aa[l] == aa[r] || ii[(left_search + 1) as usize] < ii[right_search as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", answer);
        }
    }
}