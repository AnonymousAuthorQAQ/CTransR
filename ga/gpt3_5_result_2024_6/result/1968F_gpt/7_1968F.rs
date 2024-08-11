use std::io;

const N: usize = 200000;

static mut Z: u32 = 0;

fn rand_() -> i32 {
    unsafe {
        Z = Z.wrapping_mul(3) >> 1;
        return Z as i32;
    }
}

fn srand_() {
    unsafe {
        let mut tv = libc::timeval { tv_sec: 0, tv_usec: 0 };
        libc::gettimeofday(&mut tv, std::ptr::null_mut());
        Z = tv.tv_sec ^ tv.tv_usec as u32 | 1;
    }
}

fn sort(aa: &mut [i32], ii: &mut [usize], l: usize, r: usize) {
    let mut l = l;
    let mut r = r;

    while l < r {
        let i = l;
        let mut j = l;
        let mut k = r;
        let i_ = ii[l + (rand_() % (r - l))];
        let mut tmp;

        while j < k {
            let c = if aa[ii[j]] != aa[i_] { aa[ii[j]] - aa[i_] } else { ii[j] - i_ };

            if c == 0 {
                j += 1;
            } else if c < 0 {
                tmp = ii[i]; ii[i] = ii[j]; ii[j] = tmp;
                i += 1;
                j += 1;
            } else {
                k -= 1;
                tmp = ii[j]; ii[j] = ii[k]; ii[k] = tmp;
            }
        }
        sort(aa, ii, l, i);
        l = k;
    }
}

fn search(aa: &[i32], ii: &[usize], a: i32, i: usize) -> i32 {
    let mut lower = -1;
    let mut upper = (aa.len() + 1) as i32;

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
    unsafe {
        srand_();
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: i32 = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut parts = input.trim().split_whitespace();
        let n: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut aa: Vec<i32> = input.trim().split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        
        for i in 1..=n {
            aa[i] ^= aa[i - 1];
        }

        let mut ii: Vec<usize> = (0..=n).collect();
        sort(&mut aa, &mut ii, 0, n + 1);

        for _ in 0..q {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let mut parts = input.trim().split_whitespace();
            let l: usize = parts.next().unwrap().parse().unwrap();
            let r: usize = parts.next().unwrap().parse().unwrap();

            let result = if aa[l] == aa[r] || ii[(search(&aa, &ii, aa[r], l) + 1) as usize] < ii[search(&aa, &ii, aa[l], r) as usize] {
                "YES"
            } else {
                "NO"
            };
            println!("{}", result);
        }
    }
}